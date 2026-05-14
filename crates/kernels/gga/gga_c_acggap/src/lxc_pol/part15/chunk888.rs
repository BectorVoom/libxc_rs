//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 888/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk888<F: Float>(t1181: F, t21143: F, t604: F, t7493: F, t30786: F, t30790: F, t1992: F, t5606: F, t7585: F, t7586: F, t30798: F, t30830: F, t30854: F, t1432: F, t30147: F, t30862: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34961 = t7493 * t1181 * t604 * t21143;
    let t34986 = 0.21437009059034868486e-3 * t30786;
    let t34987 = 0.28582678745379824648e-3 * t30790;
    let t34990 = t7585 * t7586 * t1992 * t5606;
    let t35004 = 0.21437009059034868486e-3 * t30798;
    let t35012 = 0.20965394859736101379e-2 * t30830;
    let t35018 = 0.25724410870841842184e-2 * t30854;
    let t35022 = t30147 * t7586 * t1992 * t1432;
    let t35024 = 0.12862205435420921092e-1 * t30862;
    (t34961, t34986, t34987, t34990, t35004, t35012, t35018, t35022, t35024)
}
