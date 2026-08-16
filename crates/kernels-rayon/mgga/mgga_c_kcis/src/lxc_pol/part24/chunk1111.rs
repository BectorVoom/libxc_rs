//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1111/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1111(t20684: f64, t2192: f64, t2197: f64, t27053: f64, t27947: f64, t27969: f64, t28235: f64, t28945: f64, t28962: f64, t28967: f64, t28974: f64, t28993: f64, t29001: f64, t29004: f64) -> (f64, f64) {
    let t29172 = t20684 * t2192;
    let t29184 = 0.11607361111111111111e-2_f64 * t28945 - t27053 - 0.34752604166666666667e-3_f64 * t29172 * t2197 - 0.34822083333333333332e-2_f64 * t28962 + 0.23214722222222222222e-2_f64 * t28967 + 0.23168402777777777778e-3_f64 * t28235 - 0.17411041666666666666e-2_f64 * t28974 + 0.23214722222222222222e-2_f64 * t27947 - 0.23214722222222222222e-2_f64 * t28993 - 0.38691203703703703703e-3_f64 * t29001 + 0.34822083333333333332e-2_f64 * t29004 - 0.23214722222222222222e-2_f64 * t27969;
    (t29172, t29184)
}
