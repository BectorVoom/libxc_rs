//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1034/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1034<F: Float>(t16222: F, t5824: F, t16210: F, t5828: F, t2198: F, t3114: F, t431: F, t442: F, t1387: F, t2059: F, t14100: F, t14082: F, t220: F, t1390: F, t3270: F, t11525: F, t11529: F, t14083: F, t14084: F, t14088: F, t14091: F, t14096: F, t14101: F, t14103: F, t14107: F, t3528: F, t3846: F, t3857: F, t429: F, t435: F, t445: F) -> (F,) {
    let t20759 = t16222 * t5824;
    let t20761 = t16210 * t5828;
    let t20763 = t3114 * t2198;
    let t20768 = t431 * t442;
    let t20781 = t1387 * t2059;
    let t20783 = t14100 * t2059;
    let t20785 = t14082 * t220;
    let t20787 = t3270 * t1390;
    let t20788 = t20787 * t220;
    let t20790 = -0.77300125e-4 * t20759 + 0.39814e-1 * t20761 - 0.10929333333333333333e-1 * t20763 + 0.1585e-2 * t435 * t11525 * t3528 + 0.10082625e-4 * t445 * t11529 * t20768 - 0.7026e-2 * t429 * t3846 - t14083 + t14084 + 0.23911438650126355246e-1 * t3857 + 0.23911438650126355246e-1 * t14088 - 0.31077233446777841256e-3 * t14091 - 0.11955719325063177623e-1 * t14096 - 0.62154466893555682512e-3 * t14101 + 0.10359077815592613752e-3 * t14103 + 0.47822877300252710492e-1 * t14107 + 0.23911438650126355246e-1 * t20781 - 0.31077233446777841256e-3 * t20783 - 0.11955719325063177623e0 * t20785 + 0.72513544709148296264e-3 * t20788;
    (t20790,)
}
