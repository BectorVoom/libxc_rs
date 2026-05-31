//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1423/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1423<F: Float>(t11478: F, t9685: F, t11474: F, t11382: F, t9489: F, t11391: F, t4576: F, t7785: F, t22754: F, t26429: F, t2829: F, t30752: F, t30760: F, t30764: F, t30767: F, t30772: F, t30777: F, t30781: F, t3753: F, t3757: F, t7806: F, t7811: F, t9533: F, t9542: F, t9558: F, t9587: F) -> (F, F, F, F, F, F) {
    let t30784 = t11478 * t9685;
    let t30787 = t11474 * t9685;
    let t30790 = t11382 * t9489;
    let t30793 = t11391 * t9489;
    let t30796 = t4576 * t7785;
    let t30799 = -F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t2829 * t30752 + F::cast_from(64.0_f64) / F::cast_from(3.0_f64) * t26429 * t30760 - F::cast_from(1936.0_f64) / F::cast_from(243.0_f64) * t3757 * t30764 - F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t22754 * t30767 + F::cast_from(5632.0_f64) / F::cast_from(2187.0_f64) * t9587 * t30772 - F::cast_from(4096.0_f64) / F::cast_from(729.0_f64) * t9587 * t30777 - F::cast_from(2560.0_f64) / F::cast_from(243.0_f64) * t3753 * t30781 - F::cast_from(112.0_f64) / F::cast_from(3.0_f64) * t9558 * t30784 + F::cast_from(32.0_f64) * t7806 * t30787 - F::cast_from(800.0_f64) / F::cast_from(9.0_f64) * t9533 * t30790 + F::cast_from(4000.0_f64) / F::cast_from(9.0_f64) * t9542 * t30793 - F::cast_from(352.0_f64) / F::cast_from(27.0_f64) * t7811 * t30796;
    (t30784, t30787, t30790, t30793, t30796, t30799)
}
