//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1118/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1118(t22724: f64, t22927: f64, t22642: f64, t22643: f64, t6907: f64, t1307: f64, t22633: f64, t22635: f64, t3886: f64, t3888: f64, t22644: f64, t81152: f64) -> (f64, f64, f64, f64) {
    let t81264 = t22724 * t22927;
    let t81267 = t22642 * t22643 * t6907;
    let t81272 = t22633 * t22635 * t3886 * t3888 * t1307;
    let t81281 = t81152 * t22644;
    (t81264, t81267, t81272, t81281)
}
