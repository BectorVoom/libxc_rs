//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2241/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2241<F: Float>(t3655: F, t8185: F, t17628: F, t7607: F, t104943: F, t17280: F, t17651: F, t17800: F, t1791: F, t26827: F, t5320: F, t7613: F, t97174: F, t97279: F, t97281: F, t97283: F, t97288: F, t97296: F) -> F {
    let t104988 = t8185 * t3655;
    let t104990 = t7607 * t17628;
    let t104992 = -F::cast_from(0.42874018118069736972e-3_f64) * t97283 * t1791 - F::cast_from(0.85748036236139473944e-3_f64) * t26827 * t5320 - F::cast_from(0.42874018118069736972e-3_f64) * t7613 * t17280 + F::cast_from(0.57165357490759649296e-3_f64) * t97279 - F::cast_from(0.28582678745379824648e-3_f64) * t97281 + F::cast_from(0.19055119163586549765e-3_f64) * t97288 + t97296 - F::cast_from(0.11433071498151929859e-2_f64) * t104943 * t17800 + F::cast_from(0.57165357490759649296e-3_f64) * t97174 * t17651 + F::cast_from(0.5081365110289746604e-3_f64) * t104988 + t104990 / F::cast_from(1296.0_f64);
    t104992
}
