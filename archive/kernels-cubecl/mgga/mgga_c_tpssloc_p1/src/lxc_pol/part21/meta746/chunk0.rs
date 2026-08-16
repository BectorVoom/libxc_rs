//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2616/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2616<F: Float>(t3506: F, t4979: F, t49850: F, t11754: F, t4889: F, t11825: F, t4993: F, t15486: F, t3490: F, t11727: F, t52835: F, t11678: F, t11697: F, t15662: F) -> (F, F, F, F, F, F) {
    let t53452 = t3506 * t49850 * t4979;
    let t53456 = t4889 * t11754;
    let t53468 = t11825 * t4993;
    let t53470 = t3490 * t15486;
    let t53472 = t52835 * t11727;
    let t53476 = t11678 * t11697 * t15662;
    (t53452, t53456, t53468, t53470, t53472, t53476)
}
