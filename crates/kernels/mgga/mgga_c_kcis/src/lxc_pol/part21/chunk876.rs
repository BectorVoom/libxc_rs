//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 876/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk876<F: Float>(t1713: F, t9568: F, t1020: F, t3178: F, t4824: F, t1092: F, t1133: F, t4772: F, t1131: F, t1096: F, t1767: F, t3190: F, t3182: F, t4823: F, t4819: F, t9532: F) -> (F, F, F, F, F, F, F, F) {
    let t14584 = t9568 * t1713;
    let t14585 = t1020 * t14584;
    let t14587 = t3178 * t4824;
    let t14588 = t1092 * t14587;
    let t14590 = t4772 * t1133;
    let t14591 = t1131 * t14590;
    let t14592 = t1096 * t14591;
    let t14593 = t1092 * t14592;
    let t14595 = t1767 * t3190;
    let t14596 = t1131 * t14595;
    let t14597 = t1096 * t14596;
    let t14598 = t1092 * t14597;
    let t14600 = t3182 * t4823;
    let t14601 = t1096 * t14600;
    let t14602 = t1092 * t14601;
    let t14604 = t9532 * t4819;
    (t14585, t14588, t14590, t14593, t14595, t14598, t14602, t14604)
}
