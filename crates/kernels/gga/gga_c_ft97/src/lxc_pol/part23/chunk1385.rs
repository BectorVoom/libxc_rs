//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1385/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1385<F: Float>(t126859: F, t28772: F, t6317: F, t113055: F, t126833: F, t126837: F, t28755: F, t113214: F, t28735: F, t28776: F, t7062: F, t31561: F, t99312: F, t24980: F, t24981: F, t31618: F, t684: F) -> (F, F, F, F, F, F, F) {
    let t127836 = t6317 * t28772 * t126859;
    let t127839 = t6317 * t113055 * t126833;
    let t127842 = t28755 * t28772 * t126837;
    let t127846 = t28735 * t113214 * t7062 * t28776;
    let t127848 = t99312 * t31561;
    let t127849 = t127848 / 18.0;
    let t127852 = t24980 * t24981 * t31618 * t684;
    (t127836, t127839, t127842, t127846, t127848, t127849, t127852)
}
