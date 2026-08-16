//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3440/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3440(t11187: f64, t11220: f64, t11224: f64, t15579: f64, t16249: f64, t16318: f64, t16592: f64, t19351: f64, t19385: f64, t19400: f64, t19415: f64, t19421: f64, t20191: f64, t20215: f64, t3047: f64, t3063: f64, t3067: f64, t3271: f64, t4747: f64, t4752: f64, t4778: f64, t4935: f64, t6393: f64) -> f64 {
    let t64592 = 0.52683593463484092788e1_f64 * t11187 * t19400 + 0.26341796731742046394e1_f64 * t20191 * t3067 - 0.26341796731742046394e1_f64 * t11224 * t19421 + 0.13170898365871023197e1_f64 * t3047 * t19385 + 0.26341796731742046394e1_f64 * t3063 * t20215 + 0.26341796731742046394e1_f64 * t11187 * t19415 - 0.13170898365871023197e1_f64 * t4752 * t16592 + 0.26341796731742046394e1_f64 * t4935 * t16318 + 0.13170898365871023197e1_f64 * t19351 * t3271 - 0.26341796731742046394e1_f64 * t4747 * t16249 - 0.13170898365871023197e1_f64 * t11220 * t6393 + 0.13170898365871023197e1_f64 * t4778 * t15579;
    t64592
}
