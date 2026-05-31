//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1430/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1430<F: Float>(t2858: F, t30955: F, t11520: F, t2824: F, t1123: F, t4535: F, t1118: F, t11548: F, t11544: F, t22750: F, t22754: F, t26409: F, t26522: F, t2838: F, t30716: F, t30723: F, t30854: F, t30919: F, t30922: F, t3680: F, t3688: F, t7637: F, t7806: F, t9533: F, t9538: F, t9542: F) -> (F, F, F, F) {
    let t30975 = t2858 * t30955;
    let t30980 = t11520 * t2824;
    let t30992 = t4535 * t1123;
    let t30993 = t1118 * t30992;
    let t30996 = t11548 * t2824;
    let t30999 = -F::cast_from(3200.0_f64) / F::cast_from(27.0_f64) * t3680 * t30723 - F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2838 * t30716 - F::cast_from(3200.0_f64) / F::cast_from(27.0_f64) * t3688 * t30723 - F::cast_from(224.0_f64) / F::cast_from(9.0_f64) * t7637 * t30975 - F::cast_from(800.0_f64) / F::cast_from(9.0_f64) * t9533 * t30919 - F::cast_from(224.0_f64) * t22750 * t30980 + F::cast_from(896.0_f64) / F::cast_from(3.0_f64) * t26409 * t30854 - F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t22754 * t30980 - F::cast_from(28672.0_f64) / F::cast_from(6561.0_f64) * t26522 * t30922 - F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t9538 * t11544 * t2824 + F::cast_from(4000.0_f64) / F::cast_from(9.0_f64) * t9542 * t30993 + F::cast_from(32.0_f64) * t7806 * t30996;
    (t30975, t30993, t30996, t30999)
}
