//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta745 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2614;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta745(t11734: f64, t15548: f64, t1174: f64, t14749: f64, t3431: f64, t1222: f64, t15723: f64, t11738: f64, t13969: f64, t15534: f64, t3514: f64, t53371: f64, t1213: f64, t15525: f64, t248: f64, t3570: f64, t11813: f64, t5018: f64, t15749: f64, t3577: f64, t45124: f64, t11835: f64, t4889: f64, t1725: f64, t2402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53378, t53387, t53389, t53397, t53399) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2614(t11734, t15548, t1174, t14749, t3431, t1222, t15723, t11738, t13969, t15534, t3514, t53371);
        let (t53404, t53406, t53410, t53433, t53440) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2615(t1213, t15525, t248, t3570, t11813, t5018, t15749, t3577, t45124, t11835, t4889, t1174, t1725, t2402);
    (t53378, t53387, t53389, t53397, t53399, t53404, t53406, t53410, t53433, t53440)
}
