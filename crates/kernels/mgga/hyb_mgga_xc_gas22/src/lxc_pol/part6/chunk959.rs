//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 959/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk959<F: Float>(t2291: F, t8776: F, t2290: F, t2312: F, t3404: F, t3423: F, t6636: F, t6641: F, t6667: F, t6716: F, t8613: F, t8616: F, t8620: F, t8754: F, t8760: F, t8763: F, t8766: F, t8770: F, t8773: F) -> (F, F) {
    let t8777 = t8776 * t2291;
    let t8780 = -F::cast_from(0.19751673498613801407e-1_f64) * t8754 - F::cast_from(0.23392894490538584828e1_f64) * t6716 * t3404 + F::cast_from(0.34631718211362927518e2_f64) * t6636 * t3423 - F::cast_from(0.23392894490538584828e1_f64) * t2290 * t8760 - F::cast_from(0.11696447245269292414e1_f64) * t2290 * t8763 - F::cast_from(0.10389515463408878255e3_f64) * t6641 * t8766 + F::cast_from(0.34631718211362927518e2_f64) * t2312 * t8770 + F::cast_from(0.17315859105681463759e2_f64) * t2312 * t8773 + F::cast_from(0.10254018858216406658e4_f64) * t6667 * t8777 + t8613 + t8616 - t8620;
    (t8777, t8780)
}
