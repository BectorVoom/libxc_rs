//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 903/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk903(t12: f64, t4872: f64, t1634: f64, t192: f64, t5093: f64, t972: f64, t1642: f64, t8: f64, t1429: f64, t439: f64, t1643: f64, t1646: f64, t2540: f64, t2543: f64, t82: f64, t87: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t6762 = 0.21687162600603479684e-1_f64 * t4872;
    let t6763 = t1634 * t192;
    let t6767 = t5093 * t972;
    let t6770 = t1642 * t8;
    let t6771 = t1429 * t439;
    let t6781 = piecewise3(t84, 0.0_f64, -8.0_f64 / 27.0_f64 * t6767 * t1643 + 16.0_f64 / 9.0_f64 * t6770 * t6771 + 4.0_f64 / 9.0_f64 * t2540 * t1646 + 8.0_f64 / 3.0_f64 * t87 * t1429 - 8.0_f64 * t2543 * t82);
    (t6762, t6763, t6767, t6770, t6771, t6781)
}
