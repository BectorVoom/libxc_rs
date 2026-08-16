//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1447/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1447<F: Float>(t1117: F, t3778: F, t1849: F, t531: F, t1620: F, t31155: F, t1160: F, t4576: F, t9656: F, t1134: F, t14765: F, t14770: F, t14775: F, t14815: F, t14818: F, t1543: F, t22754: F, t30733: F, t31483: F, t3767: F, t510: F, t7817: F, t9523: F, t9535: F, t9639: F, t9750: F) -> (F, F, F) {
    let t31512 = t1117 * t3778;
    let t31526 = t531 * t1849;
    let t31528 = t31526 * t31155 * t1620;
    let t31539 = t4576 * t1160;
    let t31540 = t31539 * t9656;
    let t31545 = F::cast_from(800.0_f64) / F::cast_from(3.0_f64) * t31512 * t9523 - F::cast_from(1600.0_f64) / F::cast_from(3.0_f64) * t510 * t9750 * t9535 - F::cast_from(8000.0_f64) / F::cast_from(3.0_f64) * t7817 * t1543 * t9523 + F::cast_from(4000.0_f64) * t30733 * t9535 + F::cast_from(5600.0_f64) * t1134 * t3767 * t9523 + F::cast_from(20000.0_f64) / F::cast_from(81.0_f64) * t14775 * t31528 + F::cast_from(5000.0_f64) / F::cast_from(27.0_f64) * t14815 * t31528 + F::cast_from(25000.0_f64) / F::cast_from(27.0_f64) * t14818 * t31528 + F::cast_from(20000.0_f64) / F::cast_from(9.0_f64) * t14765 * t31528 + F::cast_from(35000.0_f64) / F::cast_from(27.0_f64) * t14770 * t31528 - F::cast_from(64.0_f64) / F::cast_from(3.0_f64) * t22754 * t31540 + F::cast_from(256.0_f64) / F::cast_from(9.0_f64) * t9639 * t31483;
    (t31539, t31540, t31545)
}
