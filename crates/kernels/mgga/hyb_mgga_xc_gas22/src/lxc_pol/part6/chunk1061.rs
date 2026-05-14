//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1061/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1061<F: Float>(t10954: F, t949: F, t4273: F, t7070: F, t10898: F, t10913: F, t6967: F, t6969: F, t9008: F, t9012: F, t387: F, t3580: F, t3596: F, t1014: F, t10865: F, t10868: F, t10873: F, t10878: F, t10880: F, t10882: F, t10884: F, t10886: F, t260: F, t2609: F, t3591: F, t3601: F, t3606: F, t4337: F, t4341: F, t4345: F) -> (F, F, F, F, F, F) {
    let t10956 = 1.0 * t949 * t10954;
    let t10958 = 0.16081979498692535067e2 * t7070 * t4273;
    let t10963 = -t6967 + 0.12361111111111111111e-1 * t6969 + 0.24722222222222222223e-1 * t9008 - t9012 - 0.92708333333333333333e-2 * t10898 + 0.278125e-1 * t10913;
    let t10964 = t10963 * t387;
    let t10971 = t3596 * t3580;
    let t10978 = -0.11696447245269292414e1 * t3591 * t3601 - 0.17315859105681463759e2 * t1014 * t10865 - 0.34631718211362927518e2 * t1014 * t10868 - 0.10254018858216406658e4 * t1014 * t10873 + t10878 + t10880 + t10882 - t10884 + t10886 + t10956 + t10958 + 0.19751673498613801407e-1 * t260 * t10964 - 0.5848223622634646207e0 * t2609 * t4341 - 0.17315859105681463759e2 * t2609 * t4345 + 0.23392894490538584828e1 * t1014 * t10971 - 0.34631718211362927517e2 * t3591 * t3606 + 0.11696447245269292414e1 * t2609 * t4337;
    (t10956, t10958, t10963, t10964, t10971, t10978)
}
