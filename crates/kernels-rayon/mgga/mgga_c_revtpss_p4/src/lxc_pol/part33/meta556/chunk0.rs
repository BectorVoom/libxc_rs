//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1947/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1947(t33: f64, t5966: f64, t1963: f64, t25759: f64, t29598: f64, t1544: f64, t1711: f64, t5962: f64, t6079: f64, t1583: f64, t6075: f64, t1940: f64, t2403: f64, t25206: f64, t25445: f64, t27368: f64, t29705: f64, t4541: f64, t6416: f64, t7091: f64, t7783: f64, t7862: f64, t7869: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29939 = t33 * t5966;
    let t29940 = t1963 * t29939;
    let t29946 = t25759 * t29598;
    let t29949 = t1711 * t1544;
    let t29953 = t33 * t5962;
    let t29964 = t33 * t6079;
    let t29967 = t1711 * t1583;
    let t29970 = t33 * t6075;
    let t29977 = 3.0_f64 * t4541 * t29940 + 3.0_f64 * t2403 * t7783 * t7862 - 3.0_f64 * t25206 * t29946 + 3.0_f64 * t2403 * t1963 * t29949 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t29953 + t1940 * t29705 * t33 / 2.0_f64 - t1940 * t27368 * t7869 + t1940 * t7783 * t1711 + t1940 * t25445 * t29964 - t1940 * t7091 * t29967 - t1940 * t7091 * t29970 / 2.0_f64 + t1940 * t1963 * t6416 / 2.0_f64;
    (t29939, t29940, t29946, t29949, t29953, t29964, t29967, t29970, t29977)
}
