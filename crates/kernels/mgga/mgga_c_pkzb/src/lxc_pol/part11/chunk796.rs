//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 796/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk796<F: Float>(t2295: F, t3135: F, t237: F, t3113: F, t7930: F, t7979: F, t7982: F, t1201: F, t881: F, t2317: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8020 = t2295 * t3135;
    let t8028 = t237 * t3113;
    let t8038 = F::new(0.18541666666666666667e-1) * t7930;
    let t8045 = F::new(0.103295e1) * t7930;
    let t8059 = F::new(0.41678e0) * t7979;
    let t8060 = F::new(0.41678e0) * t7982;
    let t8071 = t1201 * t2295;
    let t8076 = F::new(0.60385e0) * t7930;
    let t8090 = F::new(0.33114e0) * t7979;
    let t8091 = F::new(0.33114e0) * t7982;
    let t8102 = t3113 * t881;
    let t8107 = t1201 * t2317;
    (t8020, t8028, t8038, t8045, t8059, t8060, t8071, t8076, t8090, t8091, t8102, t8107)
}
