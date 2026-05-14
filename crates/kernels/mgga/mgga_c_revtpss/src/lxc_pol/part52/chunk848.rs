//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 848/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk848<F: Float>(t5661: F, t7264: F, t25997: F, t5665: F, t1873: F, t26004: F, t5690: F, t7252: F, t25970: F, t25976: F, t26013: F, t26015: F, t27933: F, t27937: F, t27941: F, t27943: F, t27945: F, t27947: F, t27949: F) -> (F, F, F, F, F) {
    let t27951 = t7264 * t5661;
    let t27953 = t25997 * t5665;
    let t27955 = t26004 * t1873;
    let t27957 = t7252 * t5690;
    let t27959 = t27933 / 16.0 - t25970 + t25976 + 0.57165357490759649296e-4 * t26015 + 0.57165357490759649296e-4 * t27937 + t26013 + 0.85748036236139473944e-3 * t27941 + 0.17149607247227894789e-2 * t27943 - 0.42874018118069736972e-3 * t27945 + 0.17149607247227894789e-2 * t27947 - 0.17149607247227894789e-2 * t27949 - 0.42874018118069736972e-3 * t27951 - 0.25410001404642664113e-4 * t27953 + 7.0 / 144.0 * t27955 - t27957 / 48.0;
    (t27951, t27953, t27955, t27957, t27959)
}
