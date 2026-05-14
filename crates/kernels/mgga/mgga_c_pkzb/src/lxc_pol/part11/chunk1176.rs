//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1176/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1176<F: Float>(t3837: F, t8028: F, t3152: F, t898: F, t9930: F, t3153: F, t9762: F, t10168: F, t22662: F, t3820: F, t8020: F, t11541: F, t1306: F, t19339: F, t31279: F, t31281: F, t31327: F, t31329: F, t31331: F, t955: F) -> (F, F, F, F, F, F) {
    let t31618 = 0.17544670867903938621e1 * t8028 * t3837;
    let t31625 = 0.35089341735807877242e1 * t898 * t3152 * t9930;
    let t31627 = 0.35089341735807877242e1 * t9762 * t3153;
    let t31630 = 0.30762056574649219974e4 * t898 * t10168 * t22662;
    let t31633 = 0.35089341735807877242e1 * t898 * t8020 * t3820;
    let t31634 = -6.0 * t11541 * t1306 * t19339 * t955 + t31279 + t31281 - t31327 + t31329 - t31331 - t31618 + t31625 + t31627 - t31630 + t31633;
    (t31618, t31625, t31627, t31630, t31633, t31634)
}
