//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1038/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1038<F: Float>(t1555: F, t2889: F, t1161: F, t2893: F, t1535: F, t2849: F, t2850: F, t313: F, t1834: F) -> (F, F, F, F, F, F) {
    let t9681 = t1555 * t2889;
    let t9682 = t1161 * t9681;
    let t9685 = t1555 * t2893;
    let t9686 = t1161 * t9685;
    let t9689 = t2849 * t1535;
    let t9690 = t2850 * t313;
    let t9691 = t9690 * t1834;
    (t9681, t9682, t9685, t9686, t9689, t9691)
}
