//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1235/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1235<F: Float>(t2678: F, t2632: F, t2681: F, t9671: F, t2628: F, t2690: F, t812: F, t2635: F, t232: F, t40925: F, t2379: F, t2553: F, t2630: F, t2686: F, t40934: F, t40938: F, t41344: F, t41349: F, t41355: F, t41363: F, t41365: F, t817: F, t819: F, t820: F, t843: F, t9607: F, t9613: F, t9967: F, t9974: F, t9978: F, t9983: F) -> (F, F, F, F) {
    let t41367 = t2678 * t2678;
    let t41368 = t41367 * t2632;
    let t41373 = t9671 * t2681;
    let t41385 = t812 * t2628 * t2690;
    let t41386 = t41385 * t2635;
    let t41388 = t40925 * t232;
    let t41393 = -t41344 * t9978 / F::cast_from(128.0_f64) + t41349 * t819 * t820 * t40934 / F::cast_from(128.0_f64) + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t41355 - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t9974 * t819 * t820 * t40938 + F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t41363 - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t41365 + t2630 * t819 * t820 * t41368 / F::cast_from(512.0_f64) - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t41373 - F::cast_from(15.0_f64) / F::cast_from(64.0_f64) * t843 * t9607 * t820 * t2379 * t2553 + t9967 * t9983 / F::cast_from(128.0_f64) - t9613 * t2686 / F::cast_from(512.0_f64) + F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t41386 - t817 * t819 * t820 * t41388 / F::cast_from(3072.0_f64);
    (t41367, t41368, t41388, t41393)
}
