//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 898/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk898(t1280: f64, t5230: f64, t1287: f64, t5346: f64, t1774: f64, t3759: f64, t5245: f64, t354: f64, t471: f64, t1214: f64, t5351: f64, t3766: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5443 = t1280 * t5230;
    let t5446 = t5346 * t1287;
    let t5449 = t3759 * t1774;
    let t5452 = t1280 * t5245;
    let t5457 = t354 * t471;
    let t5458 = t5457 * t1214;
    let t5459 = t5351 * t5458;
    let t5462 = t3766 * t487;
    (t5443, t5446, t5449, t5452, t5457, t5458, t5459, t5462)
}
