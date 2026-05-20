//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1111;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1112;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1113;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta252<F: Float>(t1390: F, t6844: F, t828: F, t124: F, t6836: F, t800: F, t1414: F, t6816: F, t1882: F, t4003: F, t1868: F, t543: F, t5674: F, t3936: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6846, t6849, t6850, t6856, t6861) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1111::<F>(t1390, t6844, t828, t124, t6836, t800, t1414, t6816, t1882);
        let t6862 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1112::<F>(t4003, t6861);
        let (t6864, t6869) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1113::<F>(t1390, t6862, t828, t1868, t543);
        let (t6871, t6874) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1114::<F>(t5674, t6869, t3936, t543, t6861);
    (t6846, t6849, t6850, t6856, t6861, t6862, t6864, t6869, t6871, t6874)
}
