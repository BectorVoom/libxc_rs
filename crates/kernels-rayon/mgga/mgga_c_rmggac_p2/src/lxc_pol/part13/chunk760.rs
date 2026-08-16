//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 760/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk760(t2007: f64, t7939: f64, t1982: f64, t7428: f64, t7547: f64, t7542: f64, t321: f64, t7817: f64, t1550: f64, t333: f64, t903: f64, t338: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35567 = t7939 * t2007;
    let t35577 = t7547 * t7428 * t1982;
    let t35580 = t7542 * t7428 * t1982;
    let t35583 = t7817 * t321;
    let t35584 = t1550 * t35583;
    let t35586 = t7817 * t333;
    let t35587 = t903 * t35586;
    let t35589 = t338 * t830;
    (t35567, t35577, t35580, t35583, t35584, t35586, t35587, t35589)
}
