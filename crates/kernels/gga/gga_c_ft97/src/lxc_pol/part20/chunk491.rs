//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 491/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk491<F: Float>(t6008: F, t6752: F, t193: F, t1095: F, t679: F, t200: F, t6014: F, t1113: F, t203: F, t1411: F, t1127: F, t6018: F, t1109: F, t227: F, t52: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6753 = t6008 * t6752;
    let t6754 = t193 * t6753;
    let t6757 = t679 * t1095;
    let t6758 = t6757 * t200;
    let t6759 = t6014 * t6758;
    let t6762 = t203 * t1113;
    let t6763 = t6762 * t1411;
    let t6767 = t6018 * t1127;
    let t6774 = t52 * t227 * t1109;
    (t6753, t6754, t6757, t6758, t6759, t6762, t6763, t6767, t6774)
}
