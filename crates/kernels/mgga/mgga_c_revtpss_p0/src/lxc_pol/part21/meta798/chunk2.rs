//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2892/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2892<F: Float>(t11298: F, t1596: F, t11301: F, t11466: F, t1633: F, t11299: F, t1609: F, t11116: F, t11525: F, t11551: F, t11557: F, t15350: F, t15406: F, t52137: F, t52481: F, t52486: F, t52488: F, t52490: F, t52492: F, t52495: F, t52499: F, t52502: F, t52507: F, t965: F, t973: F) -> (F, F, F) {
    let t52508 = t1596 * t11298;
    let t52510 = F::cast_from(0.96491876992155210402e2_f64) * t52508 * t11301;
    let t52511 = t11466 * t1633;
    let t52514 = t11299 * t1609;
    let t52516 = F::cast_from(0.2894756309764656312e3_f64) * t52514 * t11116;
    let t52520 = F::new(6.0) * t15406 * t11551 - t52481 - t52486 - t52488 + t52490 - t52492 + t52495 - t52499 - t52502 + F::cast_from(0.35089341735807877242e1_f64) * t15350 * t11557 + t52507 + t52510 - F::cast_from(0.31168546390226634766e3_f64) * t52511 * t11525 + t52516 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t52137 * t973;
    (t52510, t52516, t52520)
}
