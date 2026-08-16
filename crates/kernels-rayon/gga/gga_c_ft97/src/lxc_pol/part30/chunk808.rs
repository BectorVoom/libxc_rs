//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 808/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk808(t1882: f64, t7669: f64, t7674: f64, t684: f64, t7629: f64, t10703: f64, t6353: f64, t6365: f64, t840: f64, t7672: f64, t824: f64, t2843: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34156 = 2.0_f64 / 9.0_f64 * t1882 * t7669;
    let t34158 = 2.0_f64 / 9.0_f64 * t1882 * t7674;
    let t34159 = t7629 * t684;
    let t34160 = t10703 * t34159;
    let t34164 = t840 * t6353 * t6365;
    let t34167 = t7672 * t824;
    let t34169 = t840 * t2843 * t34167;
    (t34156, t34158, t34159, t34160, t34164, t34167, t34169)
}
