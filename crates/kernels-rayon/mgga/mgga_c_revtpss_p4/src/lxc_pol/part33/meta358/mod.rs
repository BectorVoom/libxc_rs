//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1378;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1379;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1380;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta358(t14127: f64, t4086: f64, t543: f64, t2782: f64, t1882: f64, t4114: f64, t2482: f64, t122: f64, t4003: f64, t72: f64, t1398: f64, t676: f64, t10069: f64, t5737: f64, t5710: f64, t1432: f64, t686: f64, t136: f64, t1892: f64, t2457: f64, t3964: f64, t2435: f64, t5760: f64, t545: f64, t869: f64, t689: f64, t225: f64, t9990: f64, t213: f64, t2777: f64, t5759: f64, t2439: f64, t5659: f64, t4101: f64, t1883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14131, t14141, t14143, t14144) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1378(t14127, t4086, t543, t2782, t1882, t4114, t2482, t122, t4003, t72, t1398, t676);
        let (t14146, t14149, t14158, t14161) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1379(t14143, t14144, t14141, t10069, t5737, t5710, t72, t1432, t686, t136, t1892, t2457, t3964);
        let (t14166, t14191, t14193, t14203) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1380(t2435, t5760, t545, t5710, t869, t689, t225, t9990, t213, t2777, t5759, t2439);
        let (t14209, t14218, t14220) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1381(t1398, t1892, t4086, t543, t2782, t5659, t72, t686, t4101, t136, t1883, t2457);
    (t14131, t14146, t14149, t14158, t14161, t14166, t14191, t14193, t14203, t14209, t14218, t14220)
}
