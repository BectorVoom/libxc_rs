//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 976/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk976(t27213: f64, t7407: f64, t1956: f64, t26508: f64, t26521: f64, t26522: f64, t26529: f64, t26534: f64, t26536: f64, t26538: f64, t27199: f64, t28400: f64, t28405: f64, t28411: f64, t28418: f64, t4487: f64, t7070: f64, t7403: f64, t7420: f64) -> f64 {
    let t28422 = t27213 * t7407;
    let t28424 = -0.4336814094102599731e0_f64 * t1956 * t28400 + 0.4336814094102599731e0_f64 * t7070 * t28405 + 0.4336814094102599731e0_f64 * t27199 * t7420 - 0.26020884564615598386e1_f64 * t7070 * t28411 + t26508 + 0.13170898365871023197e1_f64 * t7403 * t4487 + t26521 - 0.12851425765524037203e-1_f64 * t26522 + 0.8673628188205199462e0_f64 * t7070 * t28418 + 0.72280234901709995518e-2_f64 * t26529 - t26534 - t26536 - t26538 + 0.72280234901709995518e-2_f64 * t28422;
    t28424
}
