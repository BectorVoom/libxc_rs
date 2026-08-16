//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1197/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1197(t481: f64, t8601: f64, t12428: f64, t792: f64, t105: f64, t3055: f64, t97: f64, t12570: f64, t42846: f64, t795: f64, t11496: f64, t2850: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43717 = t8601 * t481;
    let t43721 = t12428 * t792;
    let t43726 = t97 * t105 * t3055;
    let t43729 = t12570 * t792;
    let t43744 = t42846 * t795;
    let t43757 = t11496 * t2850;
    (t43717, t43721, t43726, t43729, t43744, t43757)
}
