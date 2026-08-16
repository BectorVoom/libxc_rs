//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta746 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2616;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta746(t3506: f64, t4979: f64, t49850: f64, t11754: f64, t4889: f64, t11825: f64, t4993: f64, t15486: f64, t3490: f64, t11727: f64, t52835: f64, t11678: f64, t11697: f64, t15662: f64, t15709: f64, t3577: f64, t1226: f64, t15764: f64, t11832: f64, t1706: f64, t11665: f64, t15608: f64, t11838: f64, t11841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53452, t53456, t53468, t53470, t53472, t53476) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2616(t3506, t4979, t49850, t11754, t4889, t11825, t4993, t15486, t3490, t11727, t52835, t11678, t11697, t15662);
        let (t53481, t53487, t53490, t53494, t53496, t53498) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2617(t11697, t15709, t3577, t1226, t15764, t11832, t1706, t11665, t15608, t11838, t4889, t11841);
    (t53452, t53456, t53468, t53470, t53472, t53476, t53481, t53487, t53490, t53494, t53496, t53498)
}
