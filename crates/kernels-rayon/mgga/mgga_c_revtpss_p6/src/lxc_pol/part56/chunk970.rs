//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 970/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk970(t1294: f64, t8931: f64, t33478: f64, t26904: f64, t3736: f64, t8937: f64, t3596: f64, t1248: f64, t1287: f64, t2142: f64, t33462: f64, t7627: f64) -> (f64, f64, f64, f64, f64) {
    let t33479 = t8931 * t1294;
    let t33480 = t33478 * t33479;
    let t33484 = t8937 * t26904 * t3736;
    let t33485 = t3596 * t8931;
    let t33487 = t33485 * t1248 * t1287;
    let t33491 = t33462 * t2142 * t7627;
    (t33480, t33484, t33485, t33487, t33491)
}
