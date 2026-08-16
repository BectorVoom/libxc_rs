//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1867/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1867<F: Float>(t1873: F, t26004: F, t5690: F, t7252: F, t25970: F, t25976: F, t26013: F, t26015: F, t27933: F, t27937: F, t27941: F, t27943: F, t27945: F, t27947: F, t27949: F, t27951: F, t27953: F) -> (F, F) {
    let t27955 = t26004 * t1873;
    let t27957 = t7252 * t5690;
    let t27959 = t27933 / F::cast_from(16.0_f64) - t25970 + t25976 + F::cast_from(0.57165357490759649296e-4_f64) * t26015 + F::cast_from(0.57165357490759649296e-4_f64) * t27937 + t26013 + F::cast_from(0.85748036236139473944e-3_f64) * t27941 + F::cast_from(0.17149607247227894789e-2_f64) * t27943 - F::cast_from(0.42874018118069736972e-3_f64) * t27945 + F::cast_from(0.17149607247227894789e-2_f64) * t27947 - F::cast_from(0.17149607247227894789e-2_f64) * t27949 - F::cast_from(0.42874018118069736972e-3_f64) * t27951 - F::cast_from(0.25410001404642664113e-4_f64) * t27953 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t27955 - t27957 / F::cast_from(48.0_f64);
    (t27955, t27959)
}
