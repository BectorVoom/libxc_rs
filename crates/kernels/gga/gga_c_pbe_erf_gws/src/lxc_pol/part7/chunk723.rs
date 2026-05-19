//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 723/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk723<F: Float>(t2003: F, t5942: F, t666: F, t678: F, t671: F, t762: F, t1989: F, t230: F, t1985: F, t226: F, t1913: F, t20: F) -> (F, F, F, F, F, F) {
    let t5944 = F::cast_from(0.11181742741110338156e-1_f64) * t2003 * t5942;
    let t5945 = t666 * t678;
    let t5948 = F::cast_from(0.11033703703703703703e-2_f64) * t762 * t671;
    let t5949 = t1989 * t230;
    let t5952 = F::new(4.0) * t226 * t1985;
    let t5953 = t1913 * t20;
    (t5944, t5945, t5948, t5949, t5952, t5953)
}
