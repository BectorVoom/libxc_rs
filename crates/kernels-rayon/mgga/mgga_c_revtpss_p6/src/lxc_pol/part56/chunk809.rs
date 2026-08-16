//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 809/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk809(t26049: f64, t686: f64, t7284: f64, t1426: f64, t2023: f64, t786: f64, t3917: f64, t25953: f64, t1445: f64, t7242: f64, t689: f64, t7275: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26050 = t26049 * t686;
    let t26051 = t7284 * t26050;
    let t26053 = t2023 * t1426;
    let t26054 = t786 * t26053;
    let t26055 = t26054 * t3917;
    let t26058 = 0.96373646535613327357e-2_f64 * t7284 * t25953;
    let t26061 = t7242 * t1445;
    let t26062 = t689 * t26061;
    let t26064 = t786 * t7275;
    (t26050, t26051, t26054, t26055, t26058, t26062, t26064)
}
