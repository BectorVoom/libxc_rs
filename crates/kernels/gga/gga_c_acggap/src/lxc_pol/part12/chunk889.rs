//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 889/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk889<F: Float>(t7884: F, t8396: F, t2137: F, t32123: F, t1619: F, t322: F, t157: F, t524: F, t929: F, t30028: F, t615: F, t8790: F, t33643: F, t315: F, t309: F, t943: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33682 = t7884 * t8396;
    let t33698 = t2137 * t32123;
    let t33699 = t1619 * t322;
    let t33706 = t524 * t929 * t157;
    let t33727 = t615 * t30028;
    let t33735 = t8790 * t929;
    let t33739 = t33643 * t157;
    let t33743 = t315 * t32123;
    let t33744 = t1619 * t309;
    let t33750 = t524 * t943;
    (t33682, t33698, t33699, t33706, t33727, t33735, t33739, t33743, t33744, t33750)
}
