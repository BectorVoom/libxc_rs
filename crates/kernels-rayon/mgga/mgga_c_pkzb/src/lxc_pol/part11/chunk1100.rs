//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1100/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1100(t2609: f64, t5152: f64, t1499: f64, t7035: f64, t16931: f64, t501: f64, t7028: f64, t496: f64, t4874: f64, t7046: f64, t4877: f64, t5331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20353 = t2609 * t5152;
    let t20358 = t7035 * t1499;
    let t20359 = 0.17544670867903938621e1_f64 * t20358;
    let t20360 = 48.0_f64 * t16931;
    let t20362 = t501 * t7028;
    let t20363 = 24.0_f64 * t20362;
    let t20365 = 24.0_f64 * t496 * t7028;
    let t20366 = t7046 * t4874;
    let t20368 = t7046 * t4877;
    let t20370 = t2609 * t5331;
    (t20353, t20359, t20360, t20363, t20365, t20366, t20368, t20370)
}
