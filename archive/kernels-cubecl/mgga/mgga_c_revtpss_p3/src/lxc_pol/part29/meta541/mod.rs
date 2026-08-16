//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1875;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta541<F: Float>(t93302: F, t95854: F, t25310: F, t26544: F, t7064: F, t95575: F, t2067: F, t41117: F, t26502: F, t786: F, t789: F, t93314: F, t7407: F, t93179: F, t25365: F, t26506: F, t25305: F, t95540: F, t10115: F, t2063: F, t213: F, t26473: F, t10982: F, t2061: F, t9646: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t95855, t95857, t95859, t95862, t95866, t95872) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1875::<F>(t93302, t95854, t25310, t26544, t7064, t95575, t2067, t41117, t26502, t786, t789, t93314);
        let (t95876, t95888, t95891, t95893, t95894, t95899) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1876::<F>(t7407, t93179, t25365, t26506, t25305, t95540, t10115, t2063, t213, t26473, t10982, t2061, t9646);
    (t95855, t95857, t95859, t95862, t95866, t95872, t95876, t95888, t95891, t95893, t95894, t95899)
}
