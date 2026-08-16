//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1010/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1010(t33660: f64, t33982: f64, t3: f64, t2042: f64, t7944: f64, t2040: f64, t7950: f64, t7953: f64, t1916: f64, t8611: f64, t1518: f64, t8453: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33983 = t33660 + t33982;
    let t33984 = t3 * t33983;
    let t33992 = param_d * t33983;
    let t33996 = t7944 * t2042;
    let t33998 = t2040 * t7950;
    let t34000 = t2040 * t7953;
    let t34003 = 6.0_f64 * t1916 * t8611;
    let t34004 = t1518 * t8453;
    (t33983, t33984, t33992, t33996, t33998, t34000, t34003, t34004)
}
