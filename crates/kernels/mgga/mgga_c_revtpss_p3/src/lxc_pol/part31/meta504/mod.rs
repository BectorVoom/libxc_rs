//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1825;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta504<F: Float>(t27212: F, t786: F, t7060: F, t7063: F, t14685: F, t1941: F, t14756: F, t4435: F, t7045: F, t4426: F, t7038: F, t25245: F, t4430: F, t1561: F, t25266: F, t25270: F, t4462: F, t4447: F, t4452: F, t1945: F, t4371: F, t807: F, t25220: F, t25232: F, t25246: F, t25256: F, t25267: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t27213, t27214, t27216, t27217, t27221, t27222, t27224, t27226, t27228) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1825::<F>(t27212, t786, t7060, t7063, t14685, t1941, t14756, t4435, t7045, t4426, t7038, t25245, t4430);
        let (t27230, t27239, t27240, t27242) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1826::<F>(t1561, t25266, t25270, t4462, t4447, t4452, t1945, t4371, t807, t25220, t25232, t25246, t25256, t25267, t27222, t27224, t27226, t27228);
    (t27213, t27214, t27216, t27217, t27221, t27228, t27230, t27239, t27240, t27242)
}
