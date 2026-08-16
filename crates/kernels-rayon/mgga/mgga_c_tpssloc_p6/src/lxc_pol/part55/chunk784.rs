//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 784/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk784(t2130: f64, t1932: f64, t2133: f64, t2132: f64, t7573: f64, t1714: f64, t460: f64, rho1: f64) -> (f64, f64, f64, f64, f64) {
    let t8025 = t2130 * rho1;
    let t8026 = 1.0_f64 / t8025;
    let t8027 = t8026 * t1932;
    let t8028 = t8027 * t2133;
    let t8031 = t2132 * t7573;
    let t8034 = t1714 * t460;
    (t8026, t8027, t8028, t8031, t8034)
}
