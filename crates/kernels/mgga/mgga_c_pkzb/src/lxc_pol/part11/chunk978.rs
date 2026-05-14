//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 978/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk978<F: Float>(t1585: F, t1588: F, t1515: F, t1528: F, t479: F, t490: F, t16200: F, t16202: F, t16205: F, t16208: F, t16210: F, t16215: F, t16217: F, t16219: F, t16221: F, t16224: F, t472: F, t491: F) -> (F, F, F, F) {
    let t16673 = t1585 * t1585;
    let t16676 = t1588 * t1588;
    let t16701 = 0.4274e0 * t479 * t1515 * t490 * t1528;
    let t16721 = 1.0 * t472 * (-0.21099166666666666667e1 * t16200 + 0.202552e2 * t16202 - 0.75019259259259259258e1 * t16205 + 0.6564185185185185185e1 * t16208 + 0.31003950617283950618e1 * t16210 + 0.68258333333333333335e-1 * t16215 - 0.10921333333333333333e1 * t16217 + 0.12134814814814814815e1 * t16219 + 0.10617962962962962963e1 * t16221 + 0.13388493827160493828e1 * t16224) * t491;
    (t16673, t16676, t16701, t16721)
}
