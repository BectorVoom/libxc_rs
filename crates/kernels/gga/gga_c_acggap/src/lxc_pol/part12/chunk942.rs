//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 942/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk942<F: Float>(t1181: F, t20775: F, t30698: F, t599: F, t4680: F, t7337: F, t8774: F, t4263: F, t7351: F, t7575: F, t1992: F, t5606: F, t7585: F, t7586: F, t4257: F, t604: F, t8463: F) -> (F, F, F, F, F) {
    let t34977 = t30698 * t1181 * t599 * t20775;
    let t34980 = t7337 * t4680 * t8774;
    let t34984 = t7575 * t1181 * t7351 * t4263;
    let t34990 = t7585 * t7586 * t1992 * t5606;
    let t34994 = t8463 * t1181 * t604 * t4257;
    (t34977, t34980, t34984, t34990, t34994)
}
