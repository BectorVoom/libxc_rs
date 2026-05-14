//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1100/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1100<F: Float>(t13917: F, t343: F, t53799: F, t53800: F, t824: F, t3209: F, t51682: F, t14121: F, t8761: F, t8806: F, t14424: F, t9371: F, t51898: F, t9243: F, t3958: F, t6148: F) -> (F, F, F, F, F, F, F) {
    let t53804 = t13917 * t53799 * t824 * t53800 * t343;
    let t53806 = t51682 * t3209;
    let t53809 = t14121 * t8761;
    let t53811 = t14121 * t8806;
    let t53816 = t13917 * t14424 * t9371;
    let t53832 = t51898 * t9243;
    let t53840 = t3958 * t6148;
    (t53804, t53806, t53809, t53811, t53816, t53832, t53840)
}
