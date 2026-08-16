//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1010/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1010(t1828: f64, t482: f64, t372: f64, t371: f64, t1715: f64, t33426: f64, t32015: f64, t2142: f64, t8190: f64) -> (f64, f64, f64, f64, f64) {
    let t34899 = t482 * t1828;
    let t34900 = t372 * t34899;
    let t34901 = t371 * t34900;
    let t34904 = t33426 * t1715;
    let t34905 = t32015 * t34904;
    let t34908 = t2142 * t8190;
    (t34899, t34901, t34904, t34905, t34908)
}
