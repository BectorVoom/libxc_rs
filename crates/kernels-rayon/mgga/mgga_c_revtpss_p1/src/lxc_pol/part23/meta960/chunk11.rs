//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3244/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3244(t5876: f64, t670: f64, t13426: f64, t1519: f64, t18227: f64, t18242: f64, t18245: f64, t21882: f64, t21891: f64, t22578: f64, t2322: f64, t27126: f64, t4248: f64, t4254: f64, t4257: f64, t4293: f64, t5517: f64, t5887: f64, t5920: f64, t5921: f64, t651: f64, t75439: f64, t7732: f64) -> (f64, f64) {
    let t85360 = t5876 * t670;
    let t85373 = -6.0_f64 * t5517 * t5920 * t651 - 12.0_f64 * t13426 * t5887 - 6.0_f64 * t1519 * t75439 - 6.0_f64 * t1519 * t85360 - 12.0_f64 * t18227 * t5887 - 6.0_f64 * t18242 * t7732 - 6.0_f64 * t18245 * t4257 - 6.0_f64 * t18245 * t4293 - 6.0_f64 * t21882 * t7732 - 12.0_f64 * t21891 * t4248 - 6.0_f64 * t22578 * t2322 - 6.0_f64 * t22578 * t4254 - 6.0_f64 * t27126 * t5921;
    (t85360, t85373)
}
