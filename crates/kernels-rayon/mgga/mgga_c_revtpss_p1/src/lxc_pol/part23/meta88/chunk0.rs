//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 613/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk613(t118: f64, t1502: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t508: f64, t511: f64, t569: f64, t651: f64, t3: f64, param_d: f64) -> (f64, f64, f64) {
    let t1913 = -t118 * t1843 - t1502 * t508 - 2.0_f64 * t1519 * t651 + t1847 * t569 + t1911 * t511;
    let t1914 = t3 * t1913;
    let t1916 = param_d * t1913;
    (t1913, t1914, t1916)
}
