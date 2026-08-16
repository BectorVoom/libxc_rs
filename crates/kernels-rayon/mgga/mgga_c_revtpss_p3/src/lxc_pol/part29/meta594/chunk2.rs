//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1991/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1991(t101522: f64, t101761: f64, t101767: f64, t101980: f64, t10416: f64, t1312: f64, t13435: f64, t13440: f64, t2055: f64, t2322: f64, t26153: f64, t27123: f64, t28219: f64, t28683: f64, t5523: f64, t7373: f64, t7889: f64, t7983: f64, t98484: f64, t98487: f64) -> f64 {
    let t102764 = 2.0_f64 * t101522 * t2055 + 2.0_f64 * t101761 * t1312 + 2.0_f64 * t10416 * t7983 + 4.0_f64 * t13435 * t7983 + 2.0_f64 * t13440 * t7983 + 2.0_f64 * t2055 * t98484 + 4.0_f64 * t2055 * t98487 + 4.0_f64 * t2322 * t28683 + 2.0_f64 * t26153 * t7889 + 4.0_f64 * t27123 * t7373 + 4.0_f64 * t28219 * t7373 + 4.0_f64 * t28683 * t5523 + 2.0_f64 * t101767 + t101980;
    t102764
}
