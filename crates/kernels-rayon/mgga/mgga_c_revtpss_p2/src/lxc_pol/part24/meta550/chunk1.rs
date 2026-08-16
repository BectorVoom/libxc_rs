//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1627/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1627(t87470: f64, t87503: f64, t87562: f64, t87579: f64, t87608: f64, t87721: f64, t87742: f64, t87783: f64, t213: f64, t234: f64, t62633: f64, t76117: f64, t76125: f64, t76134: f64, t76139: f64, t76144: f64, t76153: f64, t76158: f64, t76163: f64, t76172: f64) -> (f64, f64) {
    let t87786 = t87470 + t87503 + t87562 + t87579 + t87608 + t87721 + t87742 + t87783;
    let t87798 = -0.21951497276451705328e-1_f64 * t76117 + 0.23417857294518679245e0_f64 * t76125 + 0.65854491829355115987e0_f64 * t213 * t234 * t87786 - 0.13170898365871023197e0_f64 * t76134 + 0.65854491829355115985e-1_f64 * t76139 - 0.11708928647259339623e0_f64 * t76144 - 0.23417857294518679245e0_f64 * t76153 + 0.23417857294518679245e0_f64 * t76158 + 0.65854491829355115985e-1_f64 * t76163 + 0.78059524315062264152e-1_f64 * t62633 + 0.21951497276451705328e-1_f64 * t76172;
    (t87786, t87798)
}
