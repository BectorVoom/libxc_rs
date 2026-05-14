//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1003/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1003<F: Float>(t28624: F, t6012: F, t27544: F, t5916: F, t27543: F, t576: F, t5905: F, t1528: F, t2043: F, t27514: F, t8191: F, t5919: F, t7948: F, t5913: F, t2034: F, t491: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28625 = t28624 * t6012;
    let t28627 = t27544 * t5916;
    let t28629 = t576 * t27543;
    let t28630 = t28629 * t5905;
    let t28632 = t1528 * t2043;
    let t28634 = t27514 * t8191;
    let t28636 = t7948 * t5919;
    let t28638 = t27544 * t5913;
    let t28640 = t2034 * t491;
    (t28625, t28627, t28629, t28630, t28632, t28634, t28636, t28638, t28640)
}
