//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1139/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1139(t2185: f64, t23657: f64, t27123: f64, t5900: f64, t139212: f64, t139224: f64, t27152: f64, t32899: f64, t139213: f64, t139214: f64, t27091: f64, t147944: f64, t7239: f64, t7366: f64, t7369: f64) -> (f64, f64, f64, f64) {
    let t148435 = t23657 * t2185 * t5900 * t27123;
    let t148439 = t139212 * t139224 * t32899 * t27152;
    let t148443 = t139212 * t139213 * t139214 * t27091;
    let t148446 = t7366 * t7239 * t7369 * t147944;
    (t148435, t148439, t148443, t148446)
}
