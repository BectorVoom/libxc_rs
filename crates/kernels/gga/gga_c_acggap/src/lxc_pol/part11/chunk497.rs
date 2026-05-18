//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 497/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk497<F: Float>(t273: F, t680: F, t218: F, t761: F, t219: F, t777: F, t1072: F, t2: F, t39: F, t575: F, t661: F, t660: F) -> (F, F, F, F, F, F) {
    let t2663 = t273 * t680;
    let t2666 = t761 * t218;
    let t2667 = t2666 * t219;
    let t2668 = t777 * t2667;
    let t2669 = F::new(6.0) * t2668;
    let t2670 = t1072 * t2;
    let t2671 = t2670 * t39;
    let t2673 = t661 * t575;
    let t2674 = t660 * t2673;
    (t2663, t2666, t2669, t2671, t2673, t2674)
}
