//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1150/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1150<F: Float>(t11862: F, t5656: F, t1457: F, t3751: F, t1017: F, t86: F, t5664: F, t2006: F, t4110: F, t303: F, t3728: F, t5673: F, t2011: F, t4124: F, t1495: F, t4123: F) -> (F, F, F, F, F, F, F) {
    let t16708 = t11862 * t5656;
    let t16710 = t3751 * t1457;
    let t16712 = t86 * t1017 * t16710;
    let t16713 = t16712 * t5664;
    let t16716 = t4110 * t2006;
    let t16717 = t303 * t16716;
    let t16719 = t3728 * t5673;
    let t16720 = 0.22109259259259259258e-2 * t16719;
    let t16721 = t2011 * t4124;
    let t16722 = t1495 * t16721;
    let t16723 = t4123 * t16722;
    (t16708, t16713, t16717, t16719, t16720, t16721, t16723)
}
