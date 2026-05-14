//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 811/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk811<F: Float>(t16782: F, t418: F, t5177: F, t572: F, t587: F, t1820: F, t1866: F, t562: F, t610: F, t7703: F, t1620: F, t5064: F, t617: F, t7853: F, t1630: F, t1791: F) -> (F, F, F, F) {
    let t16787 = 32.0 / 15.0 * t587 * t16782 * t5177 * t572 * t418;
    let t16792 = 32.0 / 5.0 * t1820 * t7703 * t610 * t1866 * t562;
    let t16796 = 256.0 / 81.0 * t1620 * t7853 * t5064 * t617;
    let t16797 = t1630 * t1791;
    (t16787, t16792, t16796, t16797)
}
