//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 919/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk919<F: Float>(t2320: F, t3819: F, t889: F, t3135: F, t3139: F, t3806: F, t6233: F, t3780: F, t870: F, t1197: F, t3102: F, t3796: F) -> (F, F, F, F, F, F, F, F) {
    let t9985 = t3819 * t2320;
    let t9986 = t9985 * t889;
    let t9989 = t3139 * t3135;
    let t9992 = t3806 * t6233;
    let t9993 = t9992 * t889;
    let t10000 = t3780 * t870;
    let t10003 = t1197 * t3102;
    let t10006 = t3796 * t870;
    (t9985, t9986, t9989, t9992, t9993, t10000, t10003, t10006)
}
