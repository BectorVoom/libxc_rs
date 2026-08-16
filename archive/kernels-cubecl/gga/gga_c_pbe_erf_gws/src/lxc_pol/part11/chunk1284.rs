//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1284/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1284<F: Float>(t49745: F, t49761: F, t49763: F, t49765: F, t49767: F, t49792: F, t49793: F, t49800: F, t49802: F, t49819: F, t49826: F, t49894: F, t49895: F, t49899: F, t49903: F, t49907: F, t49912: F, t49921: F, t49928: F, t49929: F, t49931: F, t49936: F) -> (F, F) {
    let t50575 = t49745 - t49761 - t49763 + t49765 + t49767 + t49792 - t49793 - t49800 - t49802 + t49819 + t49826;
    let t50580 = t49894 + t49895 - t49899 + t49903 + t49907 + t49912 - t49921 - t49928 + t49929 + t49931 + t49936;
    (t50575, t50580)
}
