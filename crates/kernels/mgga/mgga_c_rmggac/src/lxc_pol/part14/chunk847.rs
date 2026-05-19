//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 847/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk847<F: Float>(t2012: F, t38820: F, t7349: F, t2019: F, t38815: F, t640: F, t7764: F, t2024: F, t2292: F, t26387: F, t31125: F, t34773: F, t34785: F, t34788: F, t34794: F, t38787: F, t38793: F, t38796: F, t38799: F, t38802: F, t38807: F, t38809: F, t38813: F, t38819: F, t884: F) -> F {
    let t38822 = t7349 * t2012 * t38820;
    let t38823 = F::cast_from(0.10248087766267884742e-3_f64) * t38822;
    let t38826 = t2019 * t7764 * t640 * t38815;
    let t38828 = F::cast_from(0.11974241701863808564e0_f64) * t884 * t38787 + F::cast_from(0.39914139006212695214e-1_f64) * t26387 * t2292 + F::cast_from(0.35922725105591425692e0_f64) * t38793 - F::cast_from(0.17961362552795712846e0_f64) * t38796 - F::cast_from(0.71845450211182851384e0_f64) * t38799 - F::cast_from(0.17961362552795712846e0_f64) * t38802 - t34773 - F::cast_from(0.11974241701863808564e0_f64) * t884 * t2024 * t31125 + F::cast_from(0.20455996240684006296e-1_f64) * t38807 + F::cast_from(0.59871208509319042821e-1_f64) * t884 * t38809 + F::cast_from(0.14967802127329760705e-1_f64) * t38813 + t38819 - t38823 + F::cast_from(0.30487649791575028314e-3_f64) * t38826 - t34785 + t34788 - t34794;
    t38828
}
