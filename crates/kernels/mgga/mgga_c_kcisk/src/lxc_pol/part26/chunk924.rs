//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 924/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk924<F: Float>(t443: F, t7710: F, t3859: F, t1391: F, t7706: F, t14090: F, t1349: F, t14083: F, t14084: F, t14101: F, t14107: F, t20754: F, t20756: F, t20759: F, t20761: F, t20763: F, t20781: F, t20783: F, t20785: F, t20788: F, t25312: F) -> (F,) {
    let t25538 = t443 * t7710;
    let t25540 = t3859 * t7710;
    let t25542 = t1391 * t7706;
    let t25544 = t14090 * t7706;
    let t25552 = 0.78420416666666666667e-4 * t20754 - 0.52833333333333333332e-2 * t20756 - 0.4705225e-4 * t20759 + 0.18736e-1 * t20761 - 0.21858666666666666667e-1 * t20763 - t14083 + t14084 + 0.11955719325063177623e-1 * t1349 * t25312 - 0.5179538907796306876e-4 * t1391 * t25312 - 0.11955719325063177623e-1 * t25538 + 0.10359077815592613752e-3 * t25540 + 0.23911438650126355246e-1 * t25542 - 0.31077233446777841256e-3 * t25544 - 0.31077233446777841256e-3 * t14101 + 0.23911438650126355246e-1 * t14107 + 0.47822877300252710492e-1 * t20781 - 0.62154466893555682512e-3 * t20783 - 0.47822877300252710492e-1 * t20785 + 0.41436311262370455008e-3 * t20788;
    (t25552,)
}
