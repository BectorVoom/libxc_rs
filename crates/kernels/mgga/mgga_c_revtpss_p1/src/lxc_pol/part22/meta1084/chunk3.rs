//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3928/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3928<F: Float>(t4287: F, t2289: F, t5916: F, t21877: F, t625: F, t105: F, t13475: F, t13496: F, t13503: F, t14: F, t1507: F, t21836: F, t21839: F, t21840: F, t21851: F, t21864: F, t21868: F, t21872: F, t22: F, t2344: F, t2349: F, t2350: F, t2357: F, t2359: F, t2362: F, t2363: F, t27: F, t46196: F, t49745: F, t49774: F, t5895: F, t5896: F, t5899: F, t5902: F, t656: F, t661: F, t97: F) -> (F, F, F, F) {
    let t75536 = t4287 * t4287;
    let t75540 = t2289 * t5916;
    let t75542 = t625 * t21877;
    let t75585 = -t49745 - F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t13475 * t21839 * t22 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t13496 * t21864 * t22 - F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t49774 * t21840 + F::cast_from(400.0_f64) / F::cast_from(81.0_f64) * t5902 * t2359 + F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1507 * t13503 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t656 * t21851 + F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t5902 * t2363 + F::cast_from(400.0_f64) / F::cast_from(81.0_f64) * t2344 * t5896 + F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t2344 * t5899 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t105 * t2357 * t14 * t27 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t105 * t2357 * t21872 * t661 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t105 * t21868 * t2362 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t97 * t2349 * t14 * t27 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t97 * t46196 * t5895 * t2350 + F::cast_from(100.0_f64) / F::cast_from(81.0_f64) * t656 * t21836;
    (t75536, t75540, t75542, t75585)
}
