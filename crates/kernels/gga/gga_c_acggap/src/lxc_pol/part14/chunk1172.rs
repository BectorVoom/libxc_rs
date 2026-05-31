//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1172/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1172<F: Float>(t1988: F, t9691: F, t1742: F, t1980: F, t1982: F, t1992: F, t5: F, t31570: F, t31593: F, t31598: F, t31602: F, t35775: F, t35785: F, t35789: F, t35795: F, t35798: F, t35800: F, t37719: F, t40145: F, t40147: F, t40152: F, t40156: F) -> F {
    let t40158 = t1988 * t9691;
    let t40163 = t1980 * t1982 * t5 * t1742 * t1992;
    let t40165 = F::cast_from(0.31448092289604152068e-3_f64) * t31570 - F::cast_from(0.21437009059034868486e-3_f64) * t31593 - t31598 - t31602 + t35775 + t35785 + t35789 + t37719 - t35795 + t35798 + t35800 + F::cast_from(0.17149607247227894789e-2_f64) * t40145 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t40147 + F::cast_from(0.10718504529517434243e-3_f64) * t40152 + F::cast_from(0.7145669686344956162e-4_f64) * t40156 - F::cast_from(0.31448092289604152068e-3_f64) * t40158 - F::cast_from(0.20965394859736101379e-3_f64) * t40163;
    t40165
}
