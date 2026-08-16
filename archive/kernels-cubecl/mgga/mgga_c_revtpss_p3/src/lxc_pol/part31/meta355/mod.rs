//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1371;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1372;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1373;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta355<F: Float>(t14127: F, t4086: F, t543: F, t2782: F, t1882: F, t4114: F, t2482: F, t122: F, t4003: F, t72: F, t1398: F, t676: F, t10069: F, t5737: F, t5710: F, t1432: F, t686: F, t136: F, t1892: F, t2457: F, t3964: F, t2435: F, t5760: F, t545: F, t869: F, t689: F, t225: F, t9990: F, t213: F, t2777: F, t5759: F, t2439: F, t5659: F, t4101: F, t1883: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14131, t14141, t14143, t14144) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1371::<F>(t14127, t4086, t543, t2782, t1882, t4114, t2482, t122, t4003, t72, t1398, t676);
        let (t14146, t14149, t14158, t14161) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1372::<F>(t14143, t14144, t14141, t10069, t5737, t5710, t72, t1432, t686, t136, t1892, t2457, t3964);
        let (t14166, t14191, t14193, t14203) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1373::<F>(t2435, t5760, t545, t5710, t869, t689, t225, t9990, t213, t2777, t5759, t2439);
        let (t14209, t14218, t14220) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1374::<F>(t1398, t1892, t4086, t543, t2782, t5659, t72, t686, t4101, t136, t1883, t2457);
    (t14131, t14146, t14149, t14158, t14161, t14166, t14191, t14193, t14203, t14209, t14218, t14220)
}
