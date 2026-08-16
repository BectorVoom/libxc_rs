//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1430/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1430(t2858: f64, t30955: f64, t11520: f64, t2824: f64, t1123: f64, t4535: f64, t1118: f64, t11548: f64, t11544: f64, t22750: f64, t22754: f64, t26409: f64, t26522: f64, t2838: f64, t30716: f64, t30723: f64, t30854: f64, t30919: f64, t30922: f64, t3680: f64, t3688: f64, t7637: f64, t7806: f64, t9533: f64, t9538: f64, t9542: f64) -> (f64, f64, f64, f64) {
    let t30975 = t2858 * t30955;
    let t30980 = t11520 * t2824;
    let t30992 = t4535 * t1123;
    let t30993 = t1118 * t30992;
    let t30996 = t11548 * t2824;
    let t30999 = -3200.0_f64 / 27.0_f64 * t3680 * t30723 - 88.0_f64 / 9.0_f64 * t2838 * t30716 - 3200.0_f64 / 27.0_f64 * t3688 * t30723 - 224.0_f64 / 9.0_f64 * t7637 * t30975 - 800.0_f64 / 9.0_f64 * t9533 * t30919 - 224.0_f64 * t22750 * t30980 + 896.0_f64 / 3.0_f64 * t26409 * t30854 - 32.0_f64 / 3.0_f64 * t22754 * t30980 - 28672.0_f64 / 6561.0_f64 * t26522 * t30922 - 80.0_f64 / 3.0_f64 * t9538 * t11544 * t2824 + 4000.0_f64 / 9.0_f64 * t9542 * t30993 + 32.0_f64 * t7806 * t30996;
    (t30975, t30993, t30996, t30999)
}
