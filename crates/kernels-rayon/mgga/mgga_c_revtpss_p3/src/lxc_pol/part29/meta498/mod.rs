//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1811;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1812;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta498(t28888: f64, t545: f64, t2028: f64, t689: f64, t8099: f64, t25904: f64, t25899: f64, t213: f64, t8085: f64, t1904: f64, t7492: f64, t225: f64, t27899: f64, t7515: f64, t2097: f64, t3999: f64, t14230: f64, t26304: f64, t27972: f64, t27864: f64, t1445: f64, t2027: f64, t25930: f64, t26282: f64, t26365: f64, t26366: f64, t27868: f64, t28863: f64, t561: f64, t5775: f64, t7295: f64, t7511: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28889, t28890, t28894, t28895, t28897, t28899, t28902, t28903, t28905) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1811(t28888, t545, t2028, t689, t8099, t25904, t25899, t213, t8085, t1904, t7492, t225);
        let (t28909, t28911) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1812(t27899, t7515, t2097, t3999);
        let (t28912, t28915, t28918, t28923) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1813(t14230, t28911, t26304, t27972, t27864, t1445, t1904, t2027, t213, t25930, t26282, t26365, t26366, t27868, t28863, t28890, t28895, t28897, t28899, t28903, t28905, t28909, t561, t5775, t7295, t7511);
    (t28889, t28890, t28894, t28899, t28902, t28905, t28911, t28912, t28915, t28918, t28923)
}
