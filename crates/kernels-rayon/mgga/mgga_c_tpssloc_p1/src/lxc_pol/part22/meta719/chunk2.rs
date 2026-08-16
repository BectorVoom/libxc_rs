//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2329/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2329(t1506: f64, t16723: f64, t16729: f64, t16737: f64, t16740: f64, t16746: f64, t20835: f64, t225: f64, t230: f64, t232: f64, t4219: f64, t4227: f64, t4230: f64, t5601: f64, t5605: f64, t5608: f64, t67448: f64, t67449: f64, t67451: f64, t67452: f64, t67455: f64, t67467: f64, t67491: f64, t67509: f64, t67566: f64, t68: f64, t825: f64) -> f64 {
    let t67568 = (-(t67448 + t67449 + t67451 + t67452 + t67455 + t67467 + t67491 + t67509) * t225 * t230 + 3.0_f64 * t20835 * t825 + 9.0_f64 * t16723 * t1506 - 36.0_f64 * t5601 * t68 * t4227 + 9.0_f64 * t5601 * t4230 - 36.0_f64 * t4219 * t5605 + 180.0_f64 * t16729 * t16737 - 72.0_f64 * t16729 * t16740 + 9.0_f64 * t4219 * t5608 - 36.0_f64 * t16729 * t16746 + t67566) * t232;
    t67568
}
