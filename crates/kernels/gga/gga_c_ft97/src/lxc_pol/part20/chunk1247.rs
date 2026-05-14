//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1247/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1247<F: Float>(t28719: F, t824: F, t1486: F, t193: F, t2781: F, t113320: F, t24981: F, t6317: F, t113309: F, t28772: F, t113508: F, t24980: F, t28533: F, t113316: F, t24976: F, t28755: F) -> (F, F, F, F, F, F) {
    let t113540 = t28719 * t824;
    let t113543 = t1486 * t193 * t2781 * t113540;
    let t113546 = t6317 * t24981 * t113320;
    let t113549 = t6317 * t28772 * t113309;
    let t113553 = t24980 * t24981 * t28533 * t113508;
    let t113556 = t28755 * t24976 * t113316;
    (t113540, t113543, t113546, t113549, t113553, t113556)
}
