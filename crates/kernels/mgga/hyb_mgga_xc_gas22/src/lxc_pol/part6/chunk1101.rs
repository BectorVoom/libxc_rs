//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1101/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1101<F: Float>(t3385: F, t3389: F, t4153: F, t6712: F, t828: F, t4181: F, t847: F, t1371: F, t3418: F, t4197: F, t4194: F, t10647: F, t10650: F, t10654: F, t10657: F, t10661: F, t2273: F, t2290: F, t2312: F, t3404: F, t3423: F, t6641: F, t6710: F, t8824: F, t8916: F) -> (F, F, F, F, F, F, F, F) {
    let t10778 = t3389 * t3385;
    let t10781 = t4153 * t6712;
    let t10782 = t10781 * t828;
    let t10789 = t4181 * t847;
    let t10792 = t1371 * t3418;
    let t10795 = t4197 * t847;
    let t10798 = t4194 * t847;
    let t10801 = t10647 + t10650 - t10654 - t10657 - t10661 + F::new(0.64327917994770140268e2) * t2273 * t10778 + F::new(0.2069040516770936012e4) * t6710 * t10782 - F::new(0.23392894490538584828e1) * t8824 * t3404 + F::new(0.34631718211362927517e2) * t8916 * t3423 + F::new(0.35089341735807877242e1) * t2312 * t10789 - F::new(0.23392894490538584828e1) * t2290 * t10792 - F::new(0.10389515463408878255e3) * t6641 * t10795 - F::new(0.11696447245269292414e1) * t2290 * t10798;
    (t10778, t10781, t10782, t10789, t10792, t10795, t10798, t10801)
}
