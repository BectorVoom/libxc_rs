//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1777;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1778;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1779;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1780;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta498<F: Float>(t27937: F, t27955: F, t26016: F, t26310: F, t26312: F, t26325: F, t27933: F, t27941: F, t27943: F, t27945: F, t27947: F, t27949: F, t27951: F, t27953: F, t27957: F, t28875: F, t545: F, t2028: F, t689: F, t8099: F, t25904: F, t25899: F, t213: F, t8085: F, t1904: F, t7492: F, t225: F, t27899: F, t7515: F, t2097: F, t3999: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t28887 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1777::<F>(t27937, t27955, t26016, t26310, t26312, t26325, t27933, t27941, t27943, t27945, t27947, t27949, t27951, t27953, t27957);
        let t28888 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1778::<F>(t28875, t28887);
        let (t28889, t28890, t28894) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1779::<F>(t28888, t545, t2028, t689, t8099);
        let (t28895, t28897, t28899, t28902, t28903, t28905, t28909) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1780::<F>(t25904, t28894, t25899, t213, t8085, t1904, t7492, t689, t225, t28888, t27899, t7515);
        let t28911 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1781::<F>(t2097, t3999);
    (t28888, t28889, t28890, t28894, t28895, t28897, t28899, t28902, t28903, t28905, t28909, t28911)
}
