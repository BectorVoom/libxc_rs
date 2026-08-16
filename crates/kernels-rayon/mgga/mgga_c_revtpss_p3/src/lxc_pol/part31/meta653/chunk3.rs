//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2179/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2179(t100981: f64, t106565: f64, t1113: f64, t6079: f64, t105930: f64, t106487: f64, t106496: f64, t107924: f64, t107927: f64, t107930: f64, t107934: f64, t107939: f64, t107943: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25440: f64, t25445: f64, t27158: f64, t27368: f64, t27382: f64, t27764: f64, t27802: f64, t27806: f64, t29970: f64, t6416: f64, t7087: f64) -> f64 {
    let t107947 = t100981 * t106565;
    let t107958 = t1113 * t6079;
    let t107963 = t105930 - t106496 + 2.0_f64 * t27382 * t107924 - 6.0_f64 * t27158 * t107927 - 3.0_f64 * t25206 * t107930 + 6.0_f64 * t27158 * t107934 + 6.0_f64 * t106487 * t27764 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t107939 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t107943 - 3.0_f64 * t27382 * t107947 - t1940 * t27368 * t27806 + t1940 * t7087 * t6416 / 2.0_f64 - t1940 * t25440 * t29970 / 2.0_f64 + t1940 * t25445 * t107958 - t1940 * t27368 * t27802;
    t107963
}
