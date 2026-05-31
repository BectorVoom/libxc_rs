//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1075/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1075<F: Float>(t30717: F, t1998: F, t4625: F, t2001: F, t5113: F, t5118: F, t1434: F, t7736: F, t1418: F, t7614: F, t1089: F, t598: F, t6337: F, t7679: F) -> (F, F, F, F, F, F, F) {
    let t34743 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t30717;
    let t34745 = t1998 * t4625;
    let t34746 = F::cast_from(0.17149607247227894789e-2_f64) * t34745;
    let t34747 = t2001 * t5113;
    let t34749 = t2001 * t5118;
    let t34751 = t7736 * t1434;
    let t34753 = t7614 * t1418;
    let t34754 = F::cast_from(0.32012600194825403606e-1_f64) * t34753;
    let t34757 = t598 * t1089 * t6337 * t7679;
    (t34743, t34746, t34747, t34749, t34751, t34754, t34757)
}
