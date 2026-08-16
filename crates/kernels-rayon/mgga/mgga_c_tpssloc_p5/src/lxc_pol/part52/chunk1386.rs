//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1386/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1386(t26149: f64, t8690: f64, t12725: f64, t8675: f64, t33690: f64, t6535: f64, t24932: f64, t7461: f64, t27888: f64, t25980: f64, t7266: f64, t31832: f64, t7688: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t123205 = t8690 * t26149;
    let t123206 = t12725 * t8675;
    let t123211 = t33690 * t6535;
    let t123213 = t24932 * t7461;
    let t123215 = t27888 * t7461;
    let t123217 = t7266 * t25980;
    let t123220 = t31832 * t7688;
    (t123205, t123206, t123211, t123213, t123215, t123217, t123220)
}
