//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1101/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1101(t3385: f64, t3389: f64, t4153: f64, t6712: f64, t828: f64, t4181: f64, t847: f64, t1371: f64, t3418: f64, t4197: f64, t4194: f64, t10647: f64, t10650: f64, t10654: f64, t10657: f64, t10661: f64, t2273: f64, t2290: f64, t2312: f64, t3404: f64, t3423: f64, t6641: f64, t6710: f64, t8824: f64, t8916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10778 = t3389 * t3385;
    let t10781 = t4153 * t6712;
    let t10782 = t10781 * t828;
    let t10789 = t4181 * t847;
    let t10792 = t1371 * t3418;
    let t10795 = t4197 * t847;
    let t10798 = t4194 * t847;
    let t10801 = t10647 + t10650 - t10654 - t10657 - t10661 + 0.64327917994770140268e2_f64 * t2273 * t10778 + 0.2069040516770936012e4_f64 * t6710 * t10782 - 0.23392894490538584828e1_f64 * t8824 * t3404 + 0.34631718211362927517e2_f64 * t8916 * t3423 + 0.35089341735807877242e1_f64 * t2312 * t10789 - 0.23392894490538584828e1_f64 * t2290 * t10792 - 0.10389515463408878255e3_f64 * t6641 * t10795 - 0.11696447245269292414e1_f64 * t2290 * t10798;
    (t10778, t10781, t10782, t10789, t10792, t10795, t10798, t10801)
}
