//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1075/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1075<F: Float>(t231: F, t4343: F, t2747: F, t4365: F, t10779: F, t14671: F, t6035: F, t10777: F, t14676: F, t18444: F, t4364: F, t837: F, t14894: F, t14907: F, t14925: F, t14934: F, t18527: F, t18532: F, t18618: F, t18623: F, t18629: F, t18634: F, t2745: F, t4362: F, t825: F) -> (F,) {
    let t18637 = t231 * t4343;
    let t18639 = t2747 * t4365 * t18637;
    let t18643 = t10779 * t14671 * t6035;
    let t18644 = t10777 * t18643;
    let t18647 = t2747 * t14676 * t6035;
    let t18651 = t4364 * t18444 * t837;
    let t18654 = -0.12862205435420921092e-2 * t14894 * t18527 - 0.12705000702321332056e-4 * t18532 - 0.21437009059034868486e-3 * t825 * t18618 - 0.12705000702321332056e-4 * t18623 - 0.80031500487063509015e-2 * t14907 - t14925 + 0.50820002809285328224e-4 * t14934 + 0.85748036236139473944e-3 * t2745 * t18629 + 0.85748036236139473944e-3 * t4362 * t18634 + 0.17149607247227894789e-2 * t2745 * t18639 + 0.10164000561857065645e-3 * t18644 + 0.17149607247227894789e-2 * t2745 * t18647 - 0.21437009059034868486e-3 * t2745 * t18651;
    (t18654,)
}
