//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1808;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1809;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta408(t10224: f64, t1592: f64, t973: f64, t2960: f64, t4528: f64, t1599: f64, t698: f64, t135: f64, t4542: f64, t13552: f64, t13550: f64, t13644: f64, t10295: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t13530: f64, t13534: f64, t13539: f64, t13544: f64, t13548: f64, t13557: f64, t13561: f64, t13642: f64, t13647: f64, t340: f64, t343: f64, t974: f64, t10263: f64, t10287: f64, t10290: f64, t10331: f64, t10333: f64, t10339: f64, t10342: f64, t10353: f64, t1600: f64, t4543: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13895, t13896, t13907, t13908, t13909, t13913, t13915, t13921, t13922, t13923) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1808(t10224, t1592, t973, t2960, t4528, t1599, t698, t135, t4542, t13552, t13550, t13644);
        let t13931 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1809(t10295, t10296, t10298, t10300, t10302, t13530, t13534, t13539, t13544, t13548, t13557, t13561, t13642, t13647, t13921, t13922, t13923);
        let (t13933, t13934, t13937) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1810(t13931, t340, t343, t974, t10263, t10287, t10290, t10331, t10333, t10339, t10342, t10353, t13896, t13907, t13909, t13915, t1600, t2960, t4543, t973);
    (t13895, t13908, t13913, t13931, t13933, t13934, t13937)
}
