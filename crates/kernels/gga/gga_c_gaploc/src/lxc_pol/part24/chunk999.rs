//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 999/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk999<F: Float>(t5676: F, t9944: F, t23492: F, t959: F, t23495: F, t23292: F, t787: F, t9824: F, t107: F, t408: F, t2558: F, t9823: F, t22909: F, t9820: F, t2624: F, t7419: F, t9800: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28423 = 0.59584149919750711116e-1 * t5676 * t9944;
    let t28425 = 0.29792074959875355558e-1 * t23492 * t959;
    let t28427 = 0.59584149919750711116e-1 * t23495 * t959;
    let t28435 = t787 * t23292;
    let t28437 = 0.29792074959875355558e-1 * t28435 * t9824;
    let t28438 = t107 * t408;
    let t28439 = t28438 * t2558;
    let t28441 = 0.11916829983950142223e0 * t9823 * t28439;
    let t28443 = 0.29792074959875355558e-1 * t9820 * t22909;
    let t28449 = t9800 * t2624 * t7419;
    (t28423, t28425, t28427, t28435, t28437, t28438, t28439, t28441, t28443, t28449)
}
