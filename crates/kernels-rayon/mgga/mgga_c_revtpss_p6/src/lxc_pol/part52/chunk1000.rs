//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1000/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1000(t28840: f64, t7296: f64, t72: f64, t8103: f64, t686: f64, t7284: f64, t1398: f64, t543: f64, t8085: f64, t7301: f64, t26265: f64, t5722: f64) -> (f64, f64, f64, f64, f64) {
    let t28841 = t7296 * t28840;
    let t28844 = t8103 * t72;
    let t28845 = t28844 * t686;
    let t28846 = t7284 * t28845;
    let t28849 = t8085 * t1398 * t543;
    let t28850 = t7301 * t28849;
    let t28853 = t26265 * t5722;
    (t28841, t28845, t28846, t28850, t28853)
}
