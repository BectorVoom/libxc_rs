//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 793/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk793<F: Float>(t159: F, t285: F, t5984: F, t169: F, t274: F, t301: F, t922: F, t1477: F, t535: F, t551: F, t1480: F, t1371: F, t1478: F) -> (F, F, F, F) {
    let t6032 = F::cast_from(0.67153358174671991426e-2_f64) * t5984 * t159 * t285;
    let t6036 = F::cast_from(0.92478548207158653218e0_f64) * t169 * t922 * t274 * t301;
    let t6037 = t1477 * t535;
    let t6038 = t6037 * t551;
    let t6039 = t6038 * t1480;
    let t6041 = t1478 * t1371;
    (t6032, t6036, t6039, t6041)
}
