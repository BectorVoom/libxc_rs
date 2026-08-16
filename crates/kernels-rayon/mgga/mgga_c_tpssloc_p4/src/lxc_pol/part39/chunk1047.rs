//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1047/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1047(t131: f64, t9558: f64, t205: f64, t221: f64, t2379: f64, t4128: f64, t1489: f64, t9541: f64, t4126: f64, t782: f64, t4130: f64, t12971: f64, t210: f64, t214: f64) -> (f64, f64, f64, f64, f64) {
    let t13004 = t9558 * t131;
    let t13005 = t205 * t13004;
    let t13007 = t221 * t4128 * t2379;
    let t13010 = t9541 * t1489;
    let t13012 = t782 * t4126;
    let t13014 = 0.23333333333333333332e-1_f64 * t13012 * t4130;
    let t13017 = t210 * t214 * t12971;
    (t13005, t13007, t13010, t13014, t13017)
}
