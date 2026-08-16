//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1338/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1338(t2030: f64, t47567: f64, t1444: f64, t4057: f64, t26069: f64, t94806: f64, t1426: f64, t94609: f64, t7063: f64, t7286: f64, t7289: f64, t94810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94867 = 0.81814717454467823679e-4_f64 * t47567 * t2030;
    let t94868 = t4057 * t1444;
    let t94876 = t26069 * t94806;
    let t94878 = t94609 * t1426;
    let t94879 = t7063 * t94878;
    let t94880 = t94879 * t7286;
    let t94882 = t7289 * t94810;
    (t94867, t94868, t94876, t94878, t94880, t94882)
}
