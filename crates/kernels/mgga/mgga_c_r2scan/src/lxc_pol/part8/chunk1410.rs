//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1410/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1410<F: Float>(t2620: F, t9327: F, t10106: F, t1592: F, t1632: F, t551: F, t20928: F, t20932: F, t2223: F, t25759: F, t25764: F, t25780: F, t25798: F, t25800: F, t2634: F, t30159: F, t30165: F, t30168: F, t3053: F, t33117: F, t506: F, t5109: F, t529: F, t6583: F) -> (F,) {
    let t34197 = t9327 * t2620;
    let t34201 = t1592 * t551 * t1632 * t10106;
    let t34210 = 0.51418766733487867048e0 * t25759 + 0.15425630020046360115e1 * t25764 + 0.19776387377308997907e1 * t20928 + 0.59329162131926993721e1 * t20932 - 0.2600466522016280569e0 * t6583 * t5109 * t3053 * t2634 + 0.98781737744032673976e-1 * t30159 + t25780 + 0.34672886960217074253e0 * t34197 - 0.10401866088065122276e1 * t34201 - 0.52396431978519890151e-1 * t30165 + 0.34930954652346593433e-1 * t30168 + t25798 + 0.22084125774650235182e1 * t25800 + 0.16463622957338778997e0 * t2223 * t529 * t506 * t33117;
    (t34210,)
}
