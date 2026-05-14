//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1414/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1414<F: Float>(t27400: F, t27402: F, t27406: F, t27408: F, t27411: F, t27414: F, t27417: F, t27420: F, t27423: F, t27426: F, t27429: F, t27432: F, t27436: F, t27439: F, t1306: F, t19339: F, t2461: F, t27443: F, t27447: F, t27450: F, t27452: F, t27457: F, t27459: F, t27461: F, t27463: F, t27465: F, t27467: F, t27470: F, t27472: F, t27474: F, t3936: F) -> (F, F) {
    let t28578 = -t27400 - t27402 + t27406 + t27408 + t27411 + t27414 - t27417 - t27420 - t27423 - t27426 - t27429 + t27432 + t27436 + t27439;
    let t28583 = -6.0 * t1306 * t19339 * t2461 * t3936 + t27443 + t27447 + t27450 - t27452 - t27457 + t27459 - t27461 - t27463 + t27465 - t27467 - t27470 + t27472 + t27474;
    (t28578, t28583)
}
