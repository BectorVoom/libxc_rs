//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1161/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1161(t121184: f64, t8477: f64, t32673: f64, t686: f64, t72: f64, t32710: f64, t32705: f64, t121211: f64, t32685: f64, t689: f64, t121131: f64, t121365: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122455 = t8477 * t121184;
    let t122463 = t32673 * t72 * t686;
    let t122464 = t32710 * t122463;
    let t122466 = t32705 * t122463;
    let t122468 = 0.47023883532522246276e-4_f64 * t121211;
    let t122474 = t32685 * t689;
    let t122475 = t121131 * t122474;
    let t122477 = t121365 * t122474;
    (t122455, t122464, t122466, t122468, t122475, t122477)
}
