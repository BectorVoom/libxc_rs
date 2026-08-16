//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1182/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1182(t1734: f64, t7380: f64, t7381: f64, t1886: f64, t7605: f64, t2041: f64, t5598: f64, t6167: f64, t31612: f64, t31619: f64, t31625: f64, t31627: f64, t31629: f64, t31632: f64, t31644: f64, t31646: f64, t35910: f64, t35912: f64, t35914: f64, t37757: f64, t37758: f64) -> f64 {
    let t40295 = t7380 * t7381 * t1734;
    let t40297 = t7605 * t1886;
    let t40299 = t2041 * t5598;
    let t40301 = t2041 * t6167;
    let t40305 = -t37757 - t37758 + 0.85748036236139473944e-3_f64 * t31612 + 0.94344276868812456205e-2_f64 * t31619 + 0.12862205435420921092e-2_f64 * t31625 + 0.25724410870841842184e-2_f64 * t31627 + 0.6431102717710460546e-2_f64 * t31629 - 0.40015750243531754508e-2_f64 * t31632 - t40295 / 64.0_f64 + 0.85748036236139473945e-2_f64 * t40297 - t40299 / 48.0_f64 - t40301 / 48.0_f64 - 0.11337795902333997111e-1_f64 * t31644 - 0.16006300097412701803e-1_f64 * t31646 + t35910 + t35912 + t35914;
    t40305
}
