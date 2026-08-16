//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1115/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1115(t10963: f64, t387: f64, t3580: f64, t3596: f64, t1014: f64, t10865: f64, t10868: f64, t10873: f64, t10878: f64, t10880: f64, t10882: f64, t10884: f64, t10886: f64, t10956: f64, t10958: f64, t260: f64, t2609: f64, t3591: f64, t3601: f64, t3606: f64, t4337: f64, t4341: f64, t4345: f64) -> (f64, f64, f64) {
    let t10964 = t10963 * t387;
    let t10971 = t3596 * t3580;
    let t10978 = -0.11696447245269292414e1_f64 * t3591 * t3601 - 0.17315859105681463759e2_f64 * t1014 * t10865 - 0.34631718211362927518e2_f64 * t1014 * t10868 - 0.10254018858216406658e4_f64 * t1014 * t10873 + t10878 + t10880 + t10882 - t10884 + t10886 + t10956 + t10958 + 0.19751673498613801407e-1_f64 * t260 * t10964 - 0.5848223622634646207e0_f64 * t2609 * t4341 - 0.17315859105681463759e2_f64 * t2609 * t4345 + 0.23392894490538584828e1_f64 * t1014 * t10971 - 0.34631718211362927517e2_f64 * t3591 * t3606 + 0.11696447245269292414e1_f64 * t2609 * t4337;
    (t10964, t10971, t10978)
}
