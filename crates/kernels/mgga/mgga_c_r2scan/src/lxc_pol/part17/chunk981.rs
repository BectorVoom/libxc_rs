//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 981/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk981<F: Float>(t37630: F, t37634: F, t37639: F, t10810: F, t574: F, t8066: F, t10697: F, t11669: F, t11671: F, t10698: F, t11702: F, t10885: F, t11744: F, t2834: F, t3344: F, t1615: F, t3320: F, t783: F, t978: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39492 = 0.11902492299418487743e0 * t37630;
    let t39493 = 0.35707476898255463229e0 * t37634;
    let t39494 = 0.28914548798370980346e-3 * t37639;
    let t39499 = t574 * t10810 * t8066;
    let t39502 = t10697 * t11669 * t11671;
    let t39511 = t10698 * t11702;
    let t39522 = t11744 * t10885;
    let t39548 = t2834 * t3344;
    let t39558 = t783 * t978 * t1615 * t3320;
    (t39492, t39493, t39494, t39499, t39502, t39511, t39522, t39548, t39558)
}
