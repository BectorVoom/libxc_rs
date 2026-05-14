//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1355/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1355<F: Float>(t127010: F, t31551: F, t668: F, t2665: F, t446: F, t505: F, t10683: F, t31627: F, t824: F, t24980: F, t2862: F, t28746: F, t28776: F, t4162: F, t6317: F, t2: F) -> (F, F, F, F, F, F) {
    let t127011 = t127010 / 3.0;
    let t127012 = t31551 * t668;
    let t127015 = t446 * t2665 * t127012 * t505;
    let t127019 = t446 * t10683 * t31627 * t824;
    let t127024 = t24980 * t2862 * t28746 * t28776;
    let t127027 = t6317 * t10683 * t28746 * t4162;
    let t127029 = t2 * t31551;
    (t127011, t127015, t127019, t127024, t127027, t127029)
}
