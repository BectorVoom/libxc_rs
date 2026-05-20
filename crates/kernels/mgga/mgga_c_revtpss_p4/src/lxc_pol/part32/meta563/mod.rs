//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1884;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta563<F: Float>(t14688: F, t92955: F, t4452: F, t92951: F, t14719: F, t25227: F, t2661: F, t14723: F, t25266: F, t4426: F, t1561: F, t93048: F, t14741: F, t1945: F, t807: F, t10886: F, t4416: F, t7028: F, t1549: F, t92968: F, t93001: F, t10778: F, t1941: F, t93016: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99021, t99023, t99026, t99029, t99033, t99035) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1884::<F>(t14688, t92955, t4452, t92951, t14719, t25227, t2661, t14723, t25266, t4426, t1561, t93048);
        let (t99041, t99044, t99050, t99058, t99062, t99065) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1885::<F>(t14741, t1945, t807, t10886, t4416, t7028, t1549, t92968, t93001, t10778, t1941, t93016);
    (t99021, t99023, t99026, t99029, t99033, t99035, t99041, t99044, t99050, t99058, t99062, t99065)
}
