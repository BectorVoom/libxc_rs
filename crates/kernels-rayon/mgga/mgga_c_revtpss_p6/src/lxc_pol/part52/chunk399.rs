//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 399/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk399(t1963: f64, t30: f64, t1940: f64, t343: f64, t43: f64, t136: f64, t359: f64, t365: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t1964 = t1963 * t30;
    let t1966 = t1940 * t1964 / 2.0_f64;
    let t1967 = t43 * t343;
    let t1968 = t1967 * t136;
    let t1971 = t359 * sigma0;
    let t1972 = t1971 * t365;
    (t1966, t1967, t1968, t1971, t1972)
}
