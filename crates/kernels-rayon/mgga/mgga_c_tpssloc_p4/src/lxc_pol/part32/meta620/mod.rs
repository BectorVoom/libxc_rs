//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2024;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta620(t7365: f64, t85660: f64, t131: f64, t467: f64, t50: f64, t82510: f64, t10469: f64, t461: f64, t11721: f64, t3032: f64, t3508: f64, t7368: f64, t11553: f64, t2121: f64, t2148: f64, t27561: f64, t7327: f64, t210: f64, t24810: f64, t24848: f64, t1090: f64, t24815: f64, t24594: f64, t24847: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85952, t85963, t85964, t85966, t85972, t85986) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2024(t7365, t85660, t131, t467, t50, t82510, t10469, t461, t11721, t3032, t3508, t7368);
        let (t86000, t86015, t86036, t86037, t86039, t86076) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2025(t11553, t2121, t2148, t27561, t7327, t210, t24810, t24848, t1090, t24815, t24594, t24847, t974);
    (t85952, t85963, t85964, t85966, t85972, t85986, t86000, t86015, t86036, t86037, t86039, t86076)
}
