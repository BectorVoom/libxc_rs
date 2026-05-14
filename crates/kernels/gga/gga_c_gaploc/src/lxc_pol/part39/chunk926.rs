//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 926/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk926<F: Float>(t1063: F, t38267: F, t894: F, t13725: F, t484: F, t42811: F, t42814: F, t42815: F, t42816: F, t42817: F, t42821: F, t42822: F, t42823: F, t42824: F, t197: F, t3689: F) -> (F, F) {
    let t47001 = t1063 * t894 * t38267;
    let t47003 = t484 * t13725;
    let t47005 = -t42811 - t42814 + t42815 + t42816 - t42817 - 0.28455006635676149599e-1 * t47001 - 0.15808337019820083111e-2 * t47003 - t42821 - t42822 - t42823 + t42824;
    let t47008 = t197 * t3689;
    (t47005, t47008)
}
