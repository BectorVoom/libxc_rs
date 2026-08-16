//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2143;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta615(t11365: f64, t300: f64, t1714: f64, t44583: f64, t3447: f64, t3451: f64, t44584: f64, t4904: f64, t11588: f64, t4928: f64, t461: f64, t4729: f64, t15418: f64, t11557: f64, t4889: f64, t1174: f64, t1716: f64, t2402: f64, t4930: f64, t698: f64, t44620: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51848, t51968, t51971, t51981, t52036, t52057) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2143(t11365, t300, t1714, t44583, t3447, t3451, t44584, t4904, t11588, t4928, t461, t4729);
        let (t52058, t52059, t52074, t52081, t52085, t52096) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2144(t52057, t15418, t1714, t11557, t4889, t1174, t1716, t2402, t4930, t698, t44620, t461, t60);
    (t51848, t51968, t51971, t51981, t52036, t52058, t52059, t52074, t52081, t52085, t52096)
}
