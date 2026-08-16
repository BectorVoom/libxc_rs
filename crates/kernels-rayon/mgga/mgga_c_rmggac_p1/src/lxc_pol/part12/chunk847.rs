//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 847/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk847(t2012: f64, t38820: f64, t7349: f64, t2019: f64, t38815: f64, t640: f64, t7764: f64, t2024: f64, t2292: f64, t26387: f64, t31125: f64, t34773: f64, t34785: f64, t34788: f64, t34794: f64, t38787: f64, t38793: f64, t38796: f64, t38799: f64, t38802: f64, t38807: f64, t38809: f64, t38813: f64, t38819: f64, t884: f64) -> f64 {
    let t38822 = t7349 * t2012 * t38820;
    let t38823 = 0.10248087766267884742e-3_f64 * t38822;
    let t38826 = t2019 * t7764 * t640 * t38815;
    let t38828 = 0.11974241701863808564e0_f64 * t884 * t38787 + 0.39914139006212695214e-1_f64 * t26387 * t2292 + 0.35922725105591425692e0_f64 * t38793 - 0.17961362552795712846e0_f64 * t38796 - 0.71845450211182851384e0_f64 * t38799 - 0.17961362552795712846e0_f64 * t38802 - t34773 - 0.11974241701863808564e0_f64 * t884 * t2024 * t31125 + 0.20455996240684006296e-1_f64 * t38807 + 0.59871208509319042821e-1_f64 * t884 * t38809 + 0.14967802127329760705e-1_f64 * t38813 + t38819 - t38823 + 0.30487649791575028314e-3_f64 * t38826 - t34785 + t34788 - t34794;
    t38828
}
