//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1627/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1627<F: Float>(t87470: F, t87503: F, t87562: F, t87579: F, t87608: F, t87721: F, t87742: F, t87783: F, t213: F, t234: F, t62633: F, t76117: F, t76125: F, t76134: F, t76139: F, t76144: F, t76153: F, t76158: F, t76163: F, t76172: F) -> (F, F) {
    let t87786 = t87470 + t87503 + t87562 + t87579 + t87608 + t87721 + t87742 + t87783;
    let t87798 = -F::cast_from(0.21951497276451705328e-1_f64) * t76117 + F::cast_from(0.23417857294518679245e0_f64) * t76125 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t234 * t87786 - F::cast_from(0.13170898365871023197e0_f64) * t76134 + F::cast_from(0.65854491829355115985e-1_f64) * t76139 - F::cast_from(0.11708928647259339623e0_f64) * t76144 - F::cast_from(0.23417857294518679245e0_f64) * t76153 + F::cast_from(0.23417857294518679245e0_f64) * t76158 + F::cast_from(0.65854491829355115985e-1_f64) * t76163 + F::cast_from(0.78059524315062264152e-1_f64) * t62633 + F::cast_from(0.21951497276451705328e-1_f64) * t76172;
    (t87786, t87798)
}
