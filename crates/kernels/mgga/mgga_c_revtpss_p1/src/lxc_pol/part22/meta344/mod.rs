//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1823;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1824;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta344<F: Float>(t11506: F, t315: F, t3013: F, t323: F, t3006: F, t3014: F, t2873: F, t910: F, t11132: F, t2942: F, t941: F, t2986: F, t960: F, t2979: F, t300: F, t1034: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t11507 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1823::<F>(t11506, t315);
        let t11509 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1824::<F>(t3013, t323);
        let (t11524, t11528, t11534, t11548, t11554, t11560, t11574, t11591, t11626) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1825::<F>(t3006, t3014, t2873, t910, t11132, t2942, t941, t2986, t960, t2979, t300, t1034);
    (t11507, t11509, t11524, t11528, t11534, t11548, t11554, t11560, t11574, t11591, t11626)
}
