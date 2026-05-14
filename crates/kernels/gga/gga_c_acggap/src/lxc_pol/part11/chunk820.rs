//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 820/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk820<F: Float>(t1181: F, t16325: F, t604: F, t7493: F, t7353: F, t7839: F, t1992: F, t3169: F, t7585: F, t7586: F, t3178: F, t7647: F, t3171: F, t7361: F, t1145: F, t7329: F) -> (F, F, F, F, F, F, F) {
    let t30738 = t7493 * t1181 * t604 * t16325;
    let t30744 = t7839 * t7353;
    let t30748 = t7585 * t7586 * t1992 * t3169;
    let t30750 = t7647 * t3178;
    let t30756 = t7647 * t3171;
    let t30758 = t7839 * t7361;
    let t30763 = t7329 * t1145;
    (t30738, t30744, t30748, t30750, t30756, t30758, t30763)
}
