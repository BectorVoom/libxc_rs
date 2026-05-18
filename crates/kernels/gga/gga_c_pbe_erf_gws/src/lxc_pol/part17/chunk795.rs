//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 795/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk795<F: Float>(t1477: F, t535: F, t551: F, t1480: F, t1371: F, t1478: F, t412: F, t8: F, t147: F, t1473: F, t755: F, t759: F) -> (F, F, F, F, F, F) {
    let t6037 = t1477 * t535;
    let t6038 = t6037 * t551;
    let t6039 = t6038 * t1480;
    let t6041 = t1478 * t1371;
    let t6043 = F::new(0.54655730795145295329e-4) * t6041 * t1480;
    let t6045 = F::new(1.0) / t8 / t412;
    let t6046 = t6045 * t147;
    let t6047 = t6046 * t551;
    let t6049 = F::new(0.16396719238543588599e-3) * t6047 * t1480;
    let t6050 = t1473 * t755;
    let t6053 = F::new(0.15965645347006145458e0) * t1473 * t759;
    (t6039, t6043, t6045, t6049, t6050, t6053)
}
