//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2610/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2610(t1174: f64, t18577: f64, t3440: f64, t3447: f64, t4889: f64, t4980: f64, t52953: f64, t52974: f64, t52988: f64, t52992: f64, t52994: f64, t53187: f64, t65703: f64, t66153: f64, t66155: f64, t66165: f64, t68513: f64, t71181: f64, t71185: f64) -> f64 {
    let t72842 = t66153 / 216.0_f64 + t66155 / 216.0_f64 - t65703 * t4980 / 48.0_f64 - 2.0_f64 / 27.0_f64 * t4889 * t18577 + t1174 * t3440 * t71181 / 72.0_f64 + t1174 * t3440 * t71185 / 72.0_f64 + t66165 / 144.0_f64 + t52953 - t52974 + t52988 - t52992 - t52994 - t3447 * t53187 * t68513 / 16.0_f64;
    t72842
}
