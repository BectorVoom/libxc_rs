//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1246/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1246(t25245: f64, t82031: f64, t23012: f64, t7529: f64, t22690: f64, t7520: f64, t81573: f64, t2627: f64, t7510: f64, t23030: f64, t25258: f64, t7524: f64, t81612: f64, t81613: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87068 = t82031 * t25245;
    let t87080 = t23012 * t7529;
    let t87140 = t81573 * t22690 * t7520;
    let t87142 = t2627 * t7510;
    let t87155 = t23030 * t25258;
    let t87177 = t81612 * t81613 * t7524;
    (t87068, t87080, t87140, t87142, t87155, t87177)
}
