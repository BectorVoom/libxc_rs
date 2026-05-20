//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1614/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1614<F: Float>(t40638: F, t40654: F, t50703: F, t61839: F, t61877: F, t61888: F, t61890: F, t61892: F, t61924: F, t76583: F, t76587: F, t76591: F, t76593: F, t76596: F, t76615: F, t76619: F, t76645: F, t76647: F) -> F {
    let t87579 = -F::cast_from(0.12196800674228478774e-3_f64) * t61839 - F::cast_from(0.17149607247227894789e-3_f64) * t76583 + F::cast_from(0.68598428988911579156e-3_f64) * t76587 + F::cast_from(0.30492001685571196935e-3_f64) * t76591 - F::cast_from(0.24009450146119052704e0_f64) * t76593 - F::cast_from(0.24009450146119052704e-1_f64) * t76596 + F::cast_from(0.30492001685571196936e-2_f64) * t76615 - F::cast_from(0.34299214494455789577e-3_f64) * t76619 - t40638 + t40654 + F::cast_from(0.6098400337114239387e-4_f64) * t61877 + F::cast_from(0.13011546959266941156e-2_f64) * t50703 + F::cast_from(0.5421477899694558815e-3_f64) * t61888 - F::cast_from(0.13605355082800796532e0_f64) * t61890 - F::cast_from(0.45732285992607719437e-3_f64) * t61892 - F::cast_from(0.34299214494455789577e-3_f64) * t76645 + F::cast_from(0.24009450146119052705e-1_f64) * t76647 - F::cast_from(0.18292914397043087775e-2_f64) * t61924;
    t87579
}
