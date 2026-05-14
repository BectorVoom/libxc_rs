//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1331/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1331<F: Float>(t2880: F, t4574: F, t2876: F, t4576: F, t3788: F, t11474: F, t11570: F, t11578: F, t22662: F, t26421: F, t26552: F, t26564: F, t26579: F, t2869: F, t2877: F, t2889: F, t2894: F, t30764: F, t3747: F, t4525: F, t4541: F, t4545: F, t7764: F, t9493: F, t9762: F, t9769: F) -> (F,) {
    let t31279 = t4574 * t2880;
    let t31285 = t4576 * t2876;
    let t31294 = t3788 * t2880;
    let t31303 = -1936.0 / 243.0 * t3747 * t30764 - 320.0 / 3.0 * t26421 * t11474 * t9493 + 6.0 * t31279 * t2877 - 1440.0 * t26579 * t4576 * t2869 - 4032.0 * t26564 * t31285 - 96.0 * t26552 * t31285 + 6.0 * t22662 * t4525 - 2.0 * t7764 * t4545 + 48.0 * t31294 * t11578 - 2.0 * t11570 * t2894 + t9762 * t4541 - 360.0 * t9769 * t4576 * t2889;
    (t31303,)
}
