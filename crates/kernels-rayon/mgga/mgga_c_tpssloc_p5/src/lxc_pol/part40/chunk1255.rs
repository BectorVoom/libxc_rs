//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1255/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1255(t3866: f64, t6427: f64, t6431: f64, t19735: f64, t5248: f64, t5249: f64, t16242: f64, t3805: f64, t6394: f64, t120: f64, t6414: f64, t3807: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19940 = t3866 * t6427;
    let t19942 = t3866 * t6431;
    let t19945 = t5248 * t5249 * t19735;
    let t19951 = t3805 * t16242 * t6394;
    let t19956 = t120 * t6414;
    let t19958 = t3805 * t19956 * t3807;
    (t19940, t19942, t19945, t19951, t19956, t19958)
}
