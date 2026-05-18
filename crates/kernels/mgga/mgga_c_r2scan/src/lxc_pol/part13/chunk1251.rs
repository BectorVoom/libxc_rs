//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1251/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1251<F: Float>(t11561: F, t11863: F, t11864: F, t11618: F, t11623: F, t11631: F, t11634: F, t11637: F, t12020: F, t11858: F, t11001: F, t11006: F, t11014: F, t11022: F, t11025: F, t11627: F) -> F {
    let t41104 = F::new(5.0) / F::new(8.0) * t11561;
    let t41105 = F::new(2.0) * t11863;
    let t41106 = F::new(2.0) * t11864;
    let t41107 = F::new(5.0) / F::new(8.0) * t11618;
    let t41108 = F::new(45.0) / F::new(32.0) * t11623;
    let t41109 = F::new(5.0) / F::new(8.0) * t11631;
    let t41110 = t11634 / F::new(2.0);
    let t41111 = F::new(3.0) / F::new(2.0) * t11637;
    let t41112 = F::new(2.0) * t12020;
    let t41113 = t11858 / F::new(2.0);
    let t41114 = t41104 + t41105 + t41106 + t11001 - t41107 - t11006 + t41108 - t11014 + t11627 - t41109 - t41110 + t41111 + t11022 + t41112 + t11025 + t41113;
    t41114
}
