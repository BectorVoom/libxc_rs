//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2049/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2049<F: Float>(t30313: F, t531: F, t102019: F, t102769: F, t108682: F, t109269: F, t111018: F, t1519: F, t2014: F, t22475: F, t2322: F, t25082: F, t26405: F, t27833: F, t28287: F, t28653: F, t28696: F, t28734: F, t28926: F, t28927: F, t30513: F, t30558: F, t30614: F, t4248: F, t4257: F, t4293: F, t4297: F, t5542: F, t7235: F, t7238: F, t7536: F, t7732: F, t7898: F, t7900: F, t8079: F, t95088: F) -> F {
    let t111221 = t531 * t30313;
    let t111260 = -F::cast_from(4.0_f64) * t7732 * t28696 + F::cast_from(6.0_f64) * t2014 * t102769 * t7900 + F::cast_from(3.0_f64) * t2014 * t111221 * t7238 + F::cast_from(6.0_f64) * t27833 * t8079 - F::cast_from(4.0_f64) * t28653 * t4293 - F::cast_from(4.0_f64) * t7732 * t28734 - F::cast_from(4.0_f64) * t4248 * t28734 - F::cast_from(4.0_f64) * t102019 * t1519 - F::cast_from(4.0_f64) * t111018 * t1519 - F::cast_from(4.0_f64) * t28653 * t4257 - F::cast_from(6.0_f64) * t25082 * t26405 * t108682 - F::cast_from(2.0_f64) * t2014 * t28926 * t5542 + F::cast_from(6.0_f64) * t7235 * t30614 + F::cast_from(2.0_f64) * t2014 * t7536 * t22475 + F::cast_from(2.0_f64) * t7898 * t28927 + F::cast_from(4.0_f64) * t109269 * t28287 - F::cast_from(6.0_f64) * t95088 * t30513 - F::cast_from(2.0_f64) * t2322 * t30558 - F::cast_from(4.0_f64) * t28653 * t4297;
    t111260
}
