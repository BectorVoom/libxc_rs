//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 688/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk688<F: Float>(t1989: F, t230: F, t1985: F, t226: F, t1913: F, t20: F, t2004: F, t5356: F, t5359: F, t5375: F, t5377: F, t5381: F, t5397: F, t5405: F, t5933: F, t5936: F, t5938: F, t5940: F, t5944: F, t5945: F, t5948: F) -> (F, F) {
    let t5949 = t1989 * t230;
    let t5952 = 4.0 * t226 * t1985;
    let t5953 = t1913 * t20;
    let t5954 = t5953 * t2004;
    let t5956 = t5933 + 0.32463124087094530131e0 * t5936 + 0.64926248174189060262e0 * t5938 + 0.21642082724729686754e0 * t5940 - t5944 - t5356 + t5359 + t5375 + 8.0 * t5945 + t5948 + 4.0 * t5949 + t5952 - t5377 + t5381 + 0.33545228223331014468e-1 * t5954 + t5397 + t5405;
    (t5953, t5956)
}
