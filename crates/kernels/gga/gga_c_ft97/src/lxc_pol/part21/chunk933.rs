//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 933/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk933<F: Float>(t1332: F, t4589: F, t1852: F, t6557: F, t979: F, t10969: F, t6547: F, t1286: F, t25543: F, t25546: F, t25561: F, t25588: F, t29461: F, t29465: F, t29572: F, t29578: F, t29582: F, t29586: F, t29590: F, t29594: F, t29600: F, t5501: F, t6414: F, t6423: F, t6457: F, t6461: F) -> (F, F, F, F, F, F) {
    let t29602 = t1332 * t4589;
    let t29603 = t1852 * t29602;
    let t29605 = t6557 * t979;
    let t29606 = t1852 * t29605;
    let t29608 = t10969 * t6547;
    let t29612 = t6414 * t6457 / 3.0 - t25543 / 9.0 - t25546 / 9.0 + t1286 * t29461 / 6.0 + t1286 * t29465 / 3.0 + t1286 * t29572 / 6.0 + t25561 / 27.0 + 2.0 / 9.0 * t25588 - t5501 * t29578 / 9.0 - t5501 * t29582 / 9.0 - t5501 * t29586 / 18.0 - t5501 * t29590 / 27.0 + t5501 * t29594 / 9.0 - 2.0 / 3.0 * t6414 * t6423 - 12.0 * t29600 + 4.0 * t29603 + 8.0 * t29606 + 8.0 * t29608 + t6414 * t6461 / 3.0;
    (t29602, t29603, t29605, t29606, t29608, t29612)
}
