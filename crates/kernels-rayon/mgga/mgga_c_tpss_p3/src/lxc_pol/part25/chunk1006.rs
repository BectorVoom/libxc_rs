//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1006/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1006(t13724: f64, t13752: f64, t13791: f64, t13864: f64, t219: f64, t5428: f64, t10180: f64, t1265: f64, t5432: f64, t1656: f64, t3365: f64, t4516: f64, param_beta: f64) -> (f64, f64, f64, f64, f64) {
    let t13866 = t13724 + t13752 + t13791 + t13864;
    let t13867 = param_beta * t13866;
    let t13869 = t5428 * t219;
    let t13880 = t10180 * t5432 * t1265;
    let t13884 = t3365 * t1656 * t4516;
    (t13866, t13867, t13869, t13880, t13884)
}
