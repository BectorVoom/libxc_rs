//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1424/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1424<F: Float>(t11282: F, t9696: F, t2828: F, t4574: F, t11319: F, t2849: F, t4544: F, t9691: F, t4524: F, t11266: F, t11320: F, t14815: F, t14818: F, t2821: F, t2831: F, t2838: F, t2900: F, t2940: F, t30790: F, t3661: F, t4530: F, t4568: F, t4571: F, t518: F, t7643: F, t9436: F, t9521: F, t9535: F, t9700: F) -> (F, F, F, F, F, F) {
    let t30807 = t11282 * t9696;
    let t30813 = t4574 * t2828;
    let t30818 = t11319 * t9696;
    let t30821 = t2849 * t4544;
    let t30822 = t30821 * t9691;
    let t30825 = t2849 * t4524;
    let t30826 = t30825 * t9691;
    let t30829 = t11266 * t9696;
    let t30836 = -F::cast_from(1600.0_f64) / F::cast_from(27.0_f64) * t9521 * t30790 + F::cast_from(12.0_f64) * t4571 * t2900 - F::cast_from(4.0_f64) * t2940 * t4568 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t2838 * t30807 - F::cast_from(22400.0_f64) / F::cast_from(3.0_f64) * t518 * t9436 * t9535 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t30813 * t2831 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t9700 * t11320 - F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t2821 * t30818 + F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t3661 * t30822 - F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t14815 * t30826 - F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7643 * t30829 - F::cast_from(320.0_f64) / F::cast_from(27.0_f64) * t14818 * t2849 * t4530 * t9691;
    (t30807, t30818, t30822, t30826, t30829, t30836)
}
