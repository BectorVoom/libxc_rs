//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 736/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk736<F: Float>(t1667: F, t1670: F, t1661: F, t46: F, t552: F, t1497: F, t1613: F, t542: F, t555: F, t1507: F, t540: F, t1503: F, t1499: F, t1511: F, t4952: F, t534: F, t541: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5325 = t1670 * t1667;
    let t5326 = 0.73245789224026180216e-3 * t5325;
    let t5327 = t1661 * t46;
    let t5328 = t5327 * t552;
    let t5329 = 0.54934341918019635162e-3 * t5328;
    let t5331 = t1613 * t1497 * t542;
    let t5333 = 0.35089341735807877242e1 * t555 * t5331;
    let t5335 = t1507 * t540;
    let t5336 = t1503 * t1497 * t5335;
    let t5338 = 0.51947577317044391277e2 * t555 * t5336;
    let t5339 = t1511 * t1499;
    let t5340 = 0.17544670867903938621e1 * t5339;
    let t5342 = t534 * t4952 * t541;
    (t5325, t5326, t5327, t5328, t5329, t5331, t5333, t5335, t5336, t5338, t5339, t5340, t5342)
}
