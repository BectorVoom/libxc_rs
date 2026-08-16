//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta746 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2616;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta746<F: Float>(t3506: F, t4979: F, t49850: F, t11754: F, t4889: F, t11825: F, t4993: F, t15486: F, t3490: F, t11727: F, t52835: F, t11678: F, t11697: F, t15662: F, t15709: F, t3577: F, t1226: F, t15764: F, t11832: F, t1706: F, t11665: F, t15608: F, t11838: F, t11841: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53452, t53456, t53468, t53470, t53472, t53476) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2616::<F>(t3506, t4979, t49850, t11754, t4889, t11825, t4993, t15486, t3490, t11727, t52835, t11678, t11697, t15662);
        let (t53481, t53487, t53490, t53494, t53496, t53498) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2617::<F>(t11697, t15709, t3577, t1226, t15764, t11832, t1706, t11665, t15608, t11838, t4889, t11841);
    (t53452, t53456, t53468, t53470, t53472, t53476, t53481, t53487, t53490, t53494, t53496, t53498)
}
