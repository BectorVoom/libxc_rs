//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1811;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1812;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta498<F: Float>(t28888: F, t545: F, t2028: F, t689: F, t8099: F, t25904: F, t25899: F, t213: F, t8085: F, t1904: F, t7492: F, t225: F, t27899: F, t7515: F, t2097: F, t3999: F, t14230: F, t26304: F, t27972: F, t27864: F, t1445: F, t2027: F, t25930: F, t26282: F, t26365: F, t26366: F, t27868: F, t28863: F, t561: F, t5775: F, t7295: F, t7511: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28889, t28890, t28894, t28895, t28897, t28899, t28902, t28903, t28905) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1811::<F>(t28888, t545, t2028, t689, t8099, t25904, t25899, t213, t8085, t1904, t7492, t225);
        let (t28909, t28911) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1812::<F>(t27899, t7515, t2097, t3999);
        let (t28912, t28915, t28918, t28923) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1813::<F>(t14230, t28911, t26304, t27972, t27864, t1445, t1904, t2027, t213, t25930, t26282, t26365, t26366, t27868, t28863, t28890, t28895, t28897, t28899, t28903, t28905, t28909, t561, t5775, t7295, t7511);
    (t28889, t28890, t28894, t28899, t28902, t28905, t28911, t28912, t28915, t28918, t28923)
}
