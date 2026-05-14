//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1374/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1374<F: Float>(t10159: F, t2328: F, t3147: F, t8300: F, t8021: F, t1217: F, t22394: F, t10182: F, t6337: F, t898: F, t6137: F, t9864: F, t18612: F, t9868: F, t27443: F, t27447: F, t27450: F, t27452: F, t27457: F, t27459: F) -> (F, F, F, F, F, F, F, F) {
    let t27461 = 0.11696447245269292414e1 * t2328 * t10159;
    let t27463 = 0.11696447245269292414e1 * t3147 * t8300;
    let t27465 = 0.46785788981077169656e1 * t3147 * t8021;
    let t27467 = 0.11696447245269292414e1 * t22394 * t1217;
    let t27470 = 0.17315859105681463759e2 * t898 * t10182 * t6337;
    let t27472 = 0.64327917994770140268e2 * t6137 * t9864;
    let t27474 = 0.1034520258385468006e4 * t18612 * t9868;
    let t27475 = t27443 + t27447 + t27450 - t27452 - t27457 + t27459 - t27461 - t27463 + t27465 - t27467 - t27470 + t27472 + t27474;
    (t27461, t27463, t27465, t27467, t27470, t27472, t27474, t27475)
}
