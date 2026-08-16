//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 777/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk777(t1499: f64, t1523: f64, t1525: f64, t226: f64, t255: f64, t4166: f64, t5575: f64, t5645: f64, t5648: f64, t5651: f64, t5653: f64, t5655: f64, t812: f64) -> f64 {
    let t5657 = 2.0_f64 * t1499 * t1525 - 2.0_f64 * t1523 * t4166 + t226 * t5655 + t255 * t5575 + 2.0_f64 * t5645 * t812 - 2.0_f64 * t5648 * t812 - t5651 * t812 - t5653 * t812;
    t5657
}
