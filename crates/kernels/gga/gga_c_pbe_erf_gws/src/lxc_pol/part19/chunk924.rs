//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 924/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk924<F: Float>(t3465: F, t422: F, t1809: F, t639: F, t2672: F, t34: F, t7194: F, t3411: F, t7136: F, t5312: F, t3345: F, t597: F) -> (F, F, F, F, F, F, F) {
    let t10353 = t3465 * t422;
    let t10354 = t1809 * t10353;
    let t10356 = F::new(8.0) / F::new(15.0) * t639 * t10354;
    let t10357 = t2672 * t34;
    let t10358 = t7194 * t10357;
    let t10360 = F::new(32.0) / F::new(45.0) * t639 * t10358;
    let t10362 = F::new(16.0) / F::new(45.0) * t7136 * t3411;
    let t10364 = F::new(16.0) / F::new(45.0) * t5312 * t3411;
    let t10365 = t597 * t3345;
    (t10353, t10356, t10357, t10360, t10362, t10364, t10365)
}
