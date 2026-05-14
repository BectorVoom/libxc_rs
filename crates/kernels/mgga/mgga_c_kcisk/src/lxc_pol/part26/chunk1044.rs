//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1044/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1044<F: Float>(t4374: F, t8398: F, t1591: F, t6204: F, t14962: F, t8335: F, t1588: F, t1163: F, t1312: F, t7710: F, t4400: F, t25406: F, t4406: F, t4391: F, t3952: F, t3973: F, t8323: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27742 = t4374 * t8398;
    let t27743 = t27742 * t1591;
    let t27744 = t6204 * t27743;
    let t27747 = t14962 * t8335;
    let t27748 = t27747 * t1591;
    let t27749 = t6204 * t27748;
    let t27754 = t1588 * t8398;
    let t27755 = t27754 * t1163;
    let t27756 = t1312 * t27755;
    let t27759 = t4374 * t8335;
    let t27760 = t27759 * t1163;
    let t27761 = t1312 * t27760;
    let t27764 = t7710 * t1591;
    let t27765 = t4400 * t27764;
    let t27766 = t1312 * t27765;
    let t27769 = t4406 * t25406;
    let t27770 = t1312 * t27769;
    let t27773 = t4391 * t25406;
    let t27774 = t3952 * t27773;
    let t27777 = t3973 * t8323;
    (t27744, t27749, t27756, t27761, t27764, t27766, t27770, t27774, t27777)
}
