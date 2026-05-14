//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 484/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk484<F: Float>(t2427: F, t5025: F, t14: F, t4995: F, t228: F, t231: F, t1124: F, t3799: F, t2441: F, t4917: F, t420: F, t701: F, t2446: F, t4635: F, t704: F, t2435: F, t3796: F, t3804: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5026 = t2427 * t5025;
    let t5029 = t4995 * t14;
    let t5031 = t228 * t5029 * t231;
    let t5034 = t3799 * t1124;
    let t5037 = t2441 * t4917;
    let t5038 = t420 * t5037;
    let t5039 = t701 * t5038;
    let t5041 = t2446 * t4917;
    let t5042 = t420 * t5041;
    let t5043 = t701 * t5042;
    let t5045 = t704 * t4635;
    let t5046 = t420 * t5045;
    let t5047 = t701 * t5046;
    let t5049 = 0.18727458458024691358e0 * t5031 - 0.3404992446913580247e-1 * t3796 - 0.3404992446913580247e-1 * t5034 - t2435 + 0.42562405586419753086e-2 * t3804 + 0.85124811172839506173e-2 * t5039 - 0.12768721675925925926e-1 * t5043 + 0.6384360837962962963e-2 * t5047;
    (t5026, t5031, t5034, t5037, t5038, t5039, t5041, t5042, t5043, t5045, t5046, t5047, t5049)
}
