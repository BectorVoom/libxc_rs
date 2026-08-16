//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1132/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1132(t2728: f64, t5585: f64, t1510: f64, t4295: f64, t5612: f64, t860: f64, t5617: f64, t235: f64, t5631: f64, t1499: f64, t1523: f64, t1525: f64, t226: f64, t255: f64, t4166: f64, t5575: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5645 = t2728 * t5585;
    let t5648 = t4295 * t1510;
    let t5651 = t860 * t5612;
    let t5653 = t860 * t5617;
    let t5655 = t235 * t5631;
    let t5657 = 2.0_f64 * t1499 * t1525 - 2.0_f64 * t1523 * t4166 + t226 * t5655 + t255 * t5575 + 2.0_f64 * t5645 * t812 - 2.0_f64 * t5648 * t812 - t5651 * t812 - t5653 * t812;
    (t5645, t5648, t5651, t5653, t5655, t5657)
}
