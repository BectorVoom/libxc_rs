//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 650/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk650<F: Float>(t1114: F, t6159: F, t6154: F, t1105: F, t898: F, t4423: F, t833: F, t1161: F, t2416: F, t6792: F) -> (F, F, F, F, F, F, F) {
    let t8659 = t1114 * t6159;
    let t8662 = t1114 * t6154;
    let t8713 = t898 * t1105;
    let t8746 = t1114 * t4423;
    let t8747 = t8746 * t833;
    let t8787 = t2416 * t1161;
    let t8793 = t1114 * t6792;
    (t8659, t8662, t8713, t8746, t8747, t8787, t8793)
}
