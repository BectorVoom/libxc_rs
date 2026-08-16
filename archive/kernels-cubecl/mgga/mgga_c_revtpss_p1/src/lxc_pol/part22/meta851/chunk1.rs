//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2992/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2992<F: Float>(t14619: F, t750: F, t4398: F, t9372: F, t1469: F, t2608: F, t4401: F, t606: F, t14425: F, t705: F, t39454: F, t9387: F) -> (F, F, F, F, F, F) {
    let t49864 = t14619 * t750;
    let t49866 = t4398 * t9372;
    let t49876 = t4401 * t2608 * t1469 * t606;
    let t49880 = t705 * t14425;
    let t49887 = F::cast_from(24.0_f64) * t39454;
    let t49897 = t4398 * t9387;
    (t49864, t49866, t49876, t49880, t49887, t49897)
}
