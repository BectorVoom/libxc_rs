//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 917/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk917(t112: f64, t33915: f64, t1458: f64, t2039: f64, t27188: f64, t32235: f64, t33152: f64, t33154: f64, t33234: f64, t33893: f64, t7042: f64, t7801: f64, t8446: f64, t9012: f64) -> (f64, f64) {
    let t33916 = t33915 * t112;
    let t33928 = 2.0_f64 * t1458 * t32235 + 4.0_f64 * t2039 * t27188 + 4.0_f64 * t2039 * t33234 + 4.0_f64 * t7042 * t7801 + 4.0_f64 * t7801 * t9012 + t33152 + t33154 + 2.0_f64 * t33893 + t33916 + t8446;
    (t33916, t33928)
}
