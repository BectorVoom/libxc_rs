//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1240/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1240<F: Float>(t15477: F, t44280: F, t6317: F, t6318: F, t1212: F, t2756: F, t2862: F, t28735: F, t24980: F, t25165: F, t28776: F, t28720: F, t375: F, t89: F, t25178: F, t28746: F) -> (F, F, F, F, F, F) {
    let t113434 = t6317 * t44280 * t6318 * t15477;
    let t113439 = t28735 * t2862 * t6318 * t1212 * t2756;
    let t113443 = t24980 * t2862 * t25165 * t28776;
    let t113446 = t89 * t375 * t28720;
    let t113447 = 2.0 / 3.0 * t113446;
    let t113450 = t24980 * t2862 * t28746 * t25178;
    (t113434, t113439, t113443, t113446, t113447, t113450)
}
