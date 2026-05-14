//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 504/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk504<F: Float>(t4245: F, t4308: F, t312: F, t4239: F, t1218: F, t1253: F, t301: F, t317: F, t4027: F, t4135: F, t4182: F, t4247: F, t4251: F, t4300: F, t830: F, t880: F) -> (F, F, F) {
    let t4309 = t4245 + t4308;
    let t4311 = t4239 * t312;
    let t4317 = -t1218 * t880 - t1253 * t830 - t301 * t4309 - t317 * t4027 - t317 * t4135 + 4.0 * t4182 - 2.0 * t4247 - 2.0 * t4251 - 2.0 * t4300 + 2.0 * t4311;
    (t4309, t4311, t4317)
}
