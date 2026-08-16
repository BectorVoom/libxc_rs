//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1683/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1683(t27170: f64, t510: f64, t1458: f64, t7156: f64, t1983: f64, t2040: f64, t2314: f64, t26179: f64, t27145: f64, t27147: f64, t27150: f64, t27163: f64, t4028: f64, t4034: f64, t652: f64, t7050: f64, t7057: f64, t7061: f64, t7458: f64, t7796: f64, t7806: f64) -> (f64, f64, f64) {
    let t27171 = t510 * t27170;
    let t27180 = t7156 * t1458;
    let t27183 = t1983 * t27145 - 2.0_f64 * t2040 * t26179 - 2.0_f64 * t2314 * t7796 - 2.0_f64 * t2314 * t7806 - 2.0_f64 * t27147 * t652 - 2.0_f64 * t27150 * t652 - 2.0_f64 * t27163 * t652 - 2.0_f64 * t27171 * t652 - 2.0_f64 * t27180 * t652 - 2.0_f64 * t4028 * t7061 - 2.0_f64 * t4034 * t7796 - 2.0_f64 * t4034 * t7806 - 2.0_f64 * t7050 * t7458 - 2.0_f64 * t7057 * t7458;
    (t27171, t27180, t27183)
}
