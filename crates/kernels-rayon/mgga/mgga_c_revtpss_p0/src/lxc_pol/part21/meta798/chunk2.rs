//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2892/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2892(t11298: f64, t1596: f64, t11301: f64, t11466: f64, t1633: f64, t11299: f64, t1609: f64, t11116: f64, t11525: f64, t11551: f64, t11557: f64, t15350: f64, t15406: f64, t52137: f64, t52481: f64, t52486: f64, t52488: f64, t52490: f64, t52492: f64, t52495: f64, t52499: f64, t52502: f64, t52507: f64, t965: f64, t973: f64) -> (f64, f64, f64) {
    let t52508 = t1596 * t11298;
    let t52510 = 0.96491876992155210402e2_f64 * t52508 * t11301;
    let t52511 = t11466 * t1633;
    let t52514 = t11299 * t1609;
    let t52516 = 0.2894756309764656312e3_f64 * t52514 * t11116;
    let t52520 = 6.0_f64 * t15406 * t11551 - t52481 - t52486 - t52488 + t52490 - t52492 + t52495 - t52499 - t52502 + 0.35089341735807877242e1_f64 * t15350 * t11557 + t52507 + t52510 - 0.31168546390226634766e3_f64 * t52511 * t11525 + t52516 + 0.5848223622634646207e0_f64 * t965 * t52137 * t973;
    (t52510, t52516, t52520)
}
