//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1113/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1113<F: Float>(t2672: F, t6134: F, t7372: F, t23176: F, t9820: F, t10024: F, t23348: F, t787: F, t5533: F, t883: F, t900: F, t10023: F) -> (F, F, F, F, F, F) {
    let t29014 = F::new(0.59584149919750711116e-1) * t6134 * t2672 * t7372;
    let t29016 = F::new(0.11916829983950142223e0) * t9820 * t23176;
    let t29019 = F::new(0.17875244975925213335e0) * t787 * t23348 * t10024;
    let t29020 = t883 * t5533;
    let t29021 = t900 * t29020;
    let t29023 = F::new(0.20854452471912748891e0) * t10023 * t29021;
    (t29014, t29016, t29019, t29020, t29021, t29023)
}
