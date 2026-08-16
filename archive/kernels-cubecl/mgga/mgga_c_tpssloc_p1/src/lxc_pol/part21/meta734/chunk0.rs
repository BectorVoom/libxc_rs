//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2592/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2592<F: Float>(t11536: F, t4889: F, t1174: F, t15268: F, t15281: F, t11570: F, t12652: F, t1709: F, t44633: F, t11530: F, t15273: F, t11533: F) -> (F, F, F, F, F, F, F) {
    let t52240 = t4889 * t11536;
    let t52250 = t1174 * t15281 * t15268;
    let t52271 = t11570 * t12652;
    let t52281 = t1174 * t44633 * t1709;
    let t52288 = t4889 * t11530;
    let t52296 = t1174 * t15281 * t15273;
    let t52300 = t4889 * t11533;
    (t52240, t52250, t52271, t52281, t52288, t52296, t52300)
}
