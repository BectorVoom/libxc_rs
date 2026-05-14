//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1201/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1201<F: Float>(t2057: F, t2883: F, t735: F, t7620: F, t2104: F, t2107: F, t2946: F, t54: F, t178: F, t18152: F, t5953: F, t5719: F, t2899: F, t7728: F, t774: F, t7732: F) -> (F, F, F, F, F, F, F, F) {
    let t21540 = t2057 * t2883;
    let t21542 = t735 * t7620;
    let t21567 = t2104 * t54 * t2946 * t2107;
    let t21603 = t18152 * t178;
    let t21604 = t5953 * t21603;
    let t21607 = t5719 * t21603;
    let t21611 = t2899 * t774 * t7728;
    let t21614 = t2899 * t774 * t7732;
    (t21540, t21542, t21567, t21603, t21604, t21607, t21611, t21614)
}
