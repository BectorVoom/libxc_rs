//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1046/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1046<F: Float>(t1570: F, t165: F, t3188: F, t27420: F, t1360: F, t1642: F, t1557: F, t149: F, t23413: F, t24118: F, t24119: F, t26809: F, t26811: F, t26815: F, t26817: F, t26823: F, t27192: F, t27406: F, t27411: F, t27414: F, t27417: F, t5766: F, t5772: F, t5775: F, t6584: F, t6618: F, t9439: F) -> (F, F, F, F, F, F, F, F) {
    let t27421 = t165 * t1570;
    let t27422 = t27421 * t3188;
    let t27423 = t27420 * t27422;
    let t27426 = t1642 * t1360;
    let t27427 = t165 * t1557;
    let t27428 = t27427 * t3188;
    let t27429 = t27426 * t27428;
    let t27433 = -t26809 * t26811 / 9.0 + t5772 * t26815 - t26817 * t5775 / 18.0 - t23413 * t6584 / 18.0 - t5772 * t26823 / 18.0 - t149 * t27406 + t24118 - t24119 / 18.0 + t5766 * t6618 / 6.0 - 12.0 * t9439 * t27411 + 2.0 * t27414 + t5772 * t27417 / 9.0 + t5772 * t27423 / 9.0 - t5772 * t27429 / 27.0 - 2.0 * t27192;
    (t27421, t27422, t27423, t27426, t27427, t27428, t27429, t27433)
}
