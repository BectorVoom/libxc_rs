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
    let t75585 = -t49745 - F::new(20.0) / F::new(3.0) * t13475 * t21839 * t22 + F::new(20.0) / F::new(3.0) * t13496 * t21864 * t22 - F::new(200.0) / F::new(27.0) * t49774 * t21840 + F::new(400.0) / F::new(81.0) * t5902 * t2359 + F::new(50.0) / F::new(9.0) * t1507 * t13503 - F::new(50.0) / F::new(9.0) * t656 * t21851 + F::new(200.0) / F::new(27.0) * t5902 * t2363 + F::new(400.0) / F::new(81.0) * t2344 * t5896 + F::new(200.0) / F::new(27.0) * t2344 * t5899 + F::new(20.0) / F::new(9.0) * t105 * t2357 * t14 * t27 + F::new(20.0) / F::new(9.0) * t105 * t2357 * t21872 * t661 + F::new(10.0) / F::new(9.0) * t105 * t21868 * t2362 + F::new(20.0) / F::new(9.0) * t97 * t2349 * t14 * t27 + F::new(40.0) / F::new(81.0) * t97 * t46196 * t5895 * t2350 + F::new(100.0) / F::new(81.0) * t656 * t21836;
    (t75536, t75540, t75542, t75585)
}
