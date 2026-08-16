//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2049/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2049(t30313: f64, t531: f64, t102019: f64, t102769: f64, t108682: f64, t109269: f64, t111018: f64, t1519: f64, t2014: f64, t22475: f64, t2322: f64, t25082: f64, t26405: f64, t27833: f64, t28287: f64, t28653: f64, t28696: f64, t28734: f64, t28926: f64, t28927: f64, t30513: f64, t30558: f64, t30614: f64, t4248: f64, t4257: f64, t4293: f64, t4297: f64, t5542: f64, t7235: f64, t7238: f64, t7536: f64, t7732: f64, t7898: f64, t7900: f64, t8079: f64, t95088: f64) -> f64 {
    let t111221 = t531 * t30313;
    let t111260 = -4.0_f64 * t7732 * t28696 + 6.0_f64 * t2014 * t102769 * t7900 + 3.0_f64 * t2014 * t111221 * t7238 + 6.0_f64 * t27833 * t8079 - 4.0_f64 * t28653 * t4293 - 4.0_f64 * t7732 * t28734 - 4.0_f64 * t4248 * t28734 - 4.0_f64 * t102019 * t1519 - 4.0_f64 * t111018 * t1519 - 4.0_f64 * t28653 * t4257 - 6.0_f64 * t25082 * t26405 * t108682 - 2.0_f64 * t2014 * t28926 * t5542 + 6.0_f64 * t7235 * t30614 + 2.0_f64 * t2014 * t7536 * t22475 + 2.0_f64 * t7898 * t28927 + 4.0_f64 * t109269 * t28287 - 6.0_f64 * t95088 * t30513 - 2.0_f64 * t2322 * t30558 - 4.0_f64 * t28653 * t4297;
    t111260
}
