//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1480;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1481;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta420<F: Float>(t5891: F, t8311: F, t1513: F, t31429: F, t1509: F, t8315: F, t5915: F, t109: F, t1479: F, t655: F, t31433: F, t31149: F, t5907: F, t5911: F, t31035: F, t31134: F, t31415: F, t31427: F, t31437: F, t69: F, t8258: F, t8267: F, t114: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t31626, t31629, t31633, t31636, t31640, t31643, t31646) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1480::<F>(t5891, t8311, t1513, t31429, t1509, t8315, t5915, t109, t1479, t655, t31433, t31149, t5907);
        let (t31649, t31652) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1481::<F>(t5911, t8315, t31035, t31134, t31415, t31427, t31437, t31626, t31629, t31633, t31636, t31640, t31643, t31646, t69, t8258, t8267);
        let t31653 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1482::<F>(t114, t31652);
    (t31626, t31629, t31633, t31636, t31640, t31643, t31646, t31649, t31653)
}
