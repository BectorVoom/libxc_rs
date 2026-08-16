//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 561/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk561(t4300: f64, t858: f64, t1528: f64, t259: f64, t2597: f64, t2713: f64, t4143: f64, t4145: f64, t4147: f64, t4149: f64, t4266: f64, t4268: f64, t4273: f64, t855: f64, t866: f64) -> (f64, f64) {
    let t4301 = t858 * t4300;
    let t4303 = -t1528 * t2597 - t1528 * t2713 + t259 * t4143 + t259 * t4145 + t259 * t4149 + t259 * t4266 - t4147 * t866 - t4268 * t866 + 2.0_f64 * t4273 * t855 - t4301 * t855;
    (t4301, t4303)
}
