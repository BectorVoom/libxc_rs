//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1128/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1128<F: Float>(t24554: F, t959: F, t20671: F, t22538: F, t24549: F, t11057: F, t28737: F, t10942: F, t28673: F, t2028: F, t2536: F, t787: F, t8632: F, t10007: F, t10627: F, t15482: F, t22628: F) -> (F, F, F, F, F, F) {
    let t33573 = t24554 * t959;
    let t33574 = 0.14896037479937677779e-1 * t33573;
    let t33580 = t22538 * t20671 * t24549;
    let t33581 = 0.85206502119823888168e-1 * t33580;
    let t33583 = t28737 * t11057;
    let t33584 = 0.76685851907841499352e0 * t33583;
    let t33585 = t28673 * t10942;
    let t33586 = 0.19171462976960374838e1 * t33585;
    let t33590 = 0.79445533226334281486e-1 * t787 * t2536 * t8632 * t2028;
    let t33601 = t10007 * t10627;
    let t33604 = 0.22721733898619703511e0 * t22628 * t15482 * t33601;
    (t33574, t33581, t33584, t33586, t33590, t33604)
}
