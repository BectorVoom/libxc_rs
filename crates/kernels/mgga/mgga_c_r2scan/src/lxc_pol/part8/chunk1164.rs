//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1164/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1164<F: Float>(t5286: F, t732: F, t21478: F, t234: F, t720: F, t748: F, t5272: F, t1743: F, t1818: F, t1822: F, t5266: F, t5271: F, t712: F, t159: F, t5246: F, t607: F) -> (F, F, F, F, F, F) {
    let t22134 = t732 * t5286;
    let t22139 = 0.17315859105681463759e2 * t234 * t748 * t720 * t21478;
    let t22143 = t732 * t5272;
    let t22148 = 0.61524113149298439946e4 * t234 * t1818 * t1743 * t1822;
    let t22152 = 0.36433041676861022416e6 * t234 * t5266 * t712 * t5271;
    let t22161 = t159 * t607 * t5246;
    (t22134, t22139, t22143, t22148, t22152, t22161)
}
