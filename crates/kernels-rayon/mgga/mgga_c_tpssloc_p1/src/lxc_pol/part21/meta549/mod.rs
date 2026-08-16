//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2240;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2241;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta549(t4889: f64, t4916: f64, t1653: f64, t7319: f64, t4919: f64, t15293: f64, t4928: f64, t8034: f64, t4934: f64, t1184: f64, t460: f64, t6144: f64, t1178: f64, t16558: f64, t1177: f64, t6138: f64, t11556: f64, t1174: f64, t1187: f64, t15401: f64, t15405: f64, t15422: f64, t18321: f64, t3447: f64, t4913: f64, t4931: f64, t18442: f64, t18473: f64, t18535: f64, t225: f64, t68: f64, t484: f64, t18215: f64, t3440: f64, t18211: f64, t5012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18536, t18542, t18543, t18546, t18549, t18550, t18554) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2240(t4889, t4916, t1653, t7319, t4919, t15293, t4928, t8034, t4934, t1184, t460, t6144);
        let (t18558, t18563, t18569) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2241(t18554, t4934, t1178, t16558, t1177, t1184, t460, t6138, t11556, t1174, t1187, t15401, t15405, t15422, t18321, t18536, t18543, t18546, t18550, t3447, t4889, t4913, t4931);
        let (t18571, t18572, t18573, t18574, t18577, t18580, t18583) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2242(t18442, t18473, t18535, t18569, t225, t68, t484, t18215, t3440, t18211, t1653, t5012);
    (t18542, t18549, t18554, t18558, t18563, t18571, t18572, t18573, t18574, t18577, t18580, t18583)
}
