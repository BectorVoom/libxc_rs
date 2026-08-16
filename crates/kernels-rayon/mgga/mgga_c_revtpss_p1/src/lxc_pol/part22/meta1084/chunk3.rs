//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3928/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3928(t4287: f64, t2289: f64, t5916: f64, t21877: f64, t625: f64, t105: f64, t13475: f64, t13496: f64, t13503: f64, t14: f64, t1507: f64, t21836: f64, t21839: f64, t21840: f64, t21851: f64, t21864: f64, t21868: f64, t21872: f64, t22: f64, t2344: f64, t2349: f64, t2350: f64, t2357: f64, t2359: f64, t2362: f64, t2363: f64, t27: f64, t46196: f64, t49745: f64, t49774: f64, t5895: f64, t5896: f64, t5899: f64, t5902: f64, t656: f64, t661: f64, t97: f64) -> (f64, f64, f64, f64) {
    let t75536 = t4287 * t4287;
    let t75540 = t2289 * t5916;
    let t75542 = t625 * t21877;
    let t75585 = -t49745 - 20.0_f64 / 3.0_f64 * t13475 * t21839 * t22 + 20.0_f64 / 3.0_f64 * t13496 * t21864 * t22 - 200.0_f64 / 27.0_f64 * t49774 * t21840 + 400.0_f64 / 81.0_f64 * t5902 * t2359 + 50.0_f64 / 9.0_f64 * t1507 * t13503 - 50.0_f64 / 9.0_f64 * t656 * t21851 + 200.0_f64 / 27.0_f64 * t5902 * t2363 + 400.0_f64 / 81.0_f64 * t2344 * t5896 + 200.0_f64 / 27.0_f64 * t2344 * t5899 + 20.0_f64 / 9.0_f64 * t105 * t2357 * t14 * t27 + 20.0_f64 / 9.0_f64 * t105 * t2357 * t21872 * t661 + 10.0_f64 / 9.0_f64 * t105 * t21868 * t2362 + 20.0_f64 / 9.0_f64 * t97 * t2349 * t14 * t27 + 40.0_f64 / 81.0_f64 * t97 * t46196 * t5895 * t2350 + 100.0_f64 / 81.0_f64 * t656 * t21836;
    (t75536, t75540, t75542, t75585)
}
