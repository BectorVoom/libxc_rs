//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 938/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk938<F: Float>(t32145: F, t92335: F, t136307: F, t420: F, t173: F, t32151: F, t22796: F, t5572: F, t22581: F, t32146: F, t1691: F, t32318: F) -> (F, F, F, F, F, F, F) {
    let t136356 = t92335 * t32145;
    let t136359 = t136307 * t420;
    let t136363 = t32151 * t173;
    let t136365 = t22796 * t136363 * t5572;
    let t136367 = t22581 * t173;
    let t136369 = t32146 * t136367 * t5572;
    let t136403 = t1691 * t32318;
    (t136356, t136359, t136363, t136365, t136367, t136369, t136403)
}
