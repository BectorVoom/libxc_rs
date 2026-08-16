//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1226/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1226(t34611: f64, t34616: f64, t34622: f64, t34636: f64, t34638: f64, t34640: f64, t34650: f64, t37175: f64, t37179: f64, t37180: f64, t37182: f64, t37184: f64, t39402: f64, t39406: f64, t39412: f64, t39414: f64, t39418: f64, t39422: f64) -> f64 {
    let t41638 = -t37175 + 0.22921875e-1_f64 * t39402 + 0.25724410870841842184e-2_f64 * t39406 - 0.94344276868812456205e-2_f64 * t34611 - 0.37737710747524982482e-1_f64 * t34616 - t37179 + t37180 - 0.75475421495049964964e-2_f64 * t34622 - t37182 - t37184 - 0.18868855373762491241e-2_f64 * t34636 + 0.62896184579208304138e-3_f64 * t34638 + 0.56606566121287473723e-1_f64 * t34640 - 0.34299214494455789578e-2_f64 * t39412 - 0.34299214494455789578e-2_f64 * t39414 + 0.4584375e-1_f64 * t34650 + 0.18868855373762491241e-2_f64 * t39418 - 0.37737710747524982483e-2_f64 * t39422;
    t41638
}
