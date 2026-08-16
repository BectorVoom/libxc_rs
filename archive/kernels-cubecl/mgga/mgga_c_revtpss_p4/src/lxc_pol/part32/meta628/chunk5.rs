//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2014/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2014<F: Float>(t103247: F, t103254: F, t105985: F, t105987: F, t105989: F, t105991: F, t105993: F, t105995: F, t105997: F, t105999: F, t106001: F, t106003: F) -> F {
    let t110378 = -t103247 + F::cast_from(0.34299214494455789578e-2_f64) * t105985 - t103254 - F::cast_from(0.17149607247227894789e-2_f64) * t105987 + F::cast_from(0.68598428988911579156e-2_f64) * t105989 - F::cast_from(0.51448821741683684367e-2_f64) * t105991 - F::cast_from(0.17149607247227894789e-1_f64) * t105993 + F::cast_from(0.34299214494455789578e-2_f64) * t105995 + F::cast_from(0.34299214494455789578e-2_f64) * t105997 - F::cast_from(0.68598428988911579156e-2_f64) * t105999 - F::cast_from(0.85748036236139473944e-3_f64) * t106001 + F::cast_from(0.68598428988911579156e-2_f64) * t106003;
    t110378
}
