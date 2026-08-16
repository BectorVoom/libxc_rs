//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1429/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1429(t2876: f64, t4501: f64, t2851: f64, t11319: f64, t7785: f64, t11329: f64, t11383: f64, t11386: f64, t11392: f64, t14815: f64, t22531: f64, t26194: f64, t26525: f64, t2821: f64, t2834: f64, t30710: f64, t30723: f64, t30930: f64, t30933: f64, t30936: f64, t3733: f64, t3757: f64, t7637: f64, t7811: f64, t9490: f64) -> (f64, f64, f64) {
    let t30955 = t4501 * t2876;
    let t30956 = t2851 * t30955;
    let t30961 = t11319 * t7785;
    let t30968 = -400.0_f64 / 9.0_f64 * t26525 * t11392 + 64.0_f64 / 9.0_f64 * t26194 * t11386 + 256.0_f64 / 81.0_f64 * t22531 * t30933 + 128.0_f64 / 27.0_f64 * t7811 * t30936 + 400.0_f64 / 9.0_f64 * t26525 * t11383 - 1280.0_f64 / 81.0_f64 * t3757 * t30930 + 616.0_f64 / 9.0_f64 * t7637 * t30710 - 64.0_f64 / 27.0_f64 * t14815 * t30956 + 800.0_f64 / 27.0_f64 * t11329 * t9490 + 88.0_f64 / 27.0_f64 * t2821 * t30961 - 3200.0_f64 / 81.0_f64 * t3733 * t30723 + 88.0_f64 / 9.0_f64 * t2834 * t30961;
    (t30955, t30956, t30968)
}
