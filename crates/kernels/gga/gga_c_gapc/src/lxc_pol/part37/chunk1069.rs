//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1069/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1069<F: Float>(t11451: F, t11518: F, t20897: F, t11517: F, t33490: F, t34535: F, t5117: F, t11438: F, t19686: F, t3021: F, t11442: F, t19671: F, t11322: F, t611: F, t9386: F, t11483: F, t11485: F, t1846: F) -> (F, F, F, F, F, F, F) {
    let t34654 = t11518 * t11451 * t20897;
    let t34656 = t11517 * t33490;
    let t34658 = t34656 * t34535 * t5117;
    let t34661 = t11438 * t3021 * t19686;
    let t34663 = t19671 * t11442;
    let t34666 = t611 * t9386 * t11322;
    let t34669 = t1846 * t11483 * t11485;
    (t34654, t34656, t34658, t34661, t34663, t34666, t34669)
}
