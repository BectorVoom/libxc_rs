//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 761/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk761<F: Float>(t4292: F, t508: F, t1843: F, t670: F, t2616: F, t2524: F, t1534: F, t72: F, t757: F, t1469: F, t750: F, t706: F, t190: F, t4186: F, t1531: F, t705: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4293 = t508 * t4292;
    let t4297 = t1843 * t670;
    let t4300 = 4.0 * t2616;
    let t4301 = 0.5848223622634646207e0 * t2524;
    let t4302 = t1534 * t72;
    let t4303 = t4302 * t757;
    let t4304 = 0.18311447306006545054e-3 * t4303;
    let t4305 = t750 * t1469;
    let t4306 = t706 * t4305;
    let t4307 = 4.0 * t4306;
    let t4308 = t190 * t4186;
    let t4310 = 4.0 * t706 * t4308;
    let t4311 = t705 * t1531;
    (t4293, t4297, t4300, t4301, t4302, t4304, t4305, t4307, t4308, t4310, t4311)
}
