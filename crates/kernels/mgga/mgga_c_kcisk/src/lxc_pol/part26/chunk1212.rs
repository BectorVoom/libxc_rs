//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1212/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1212<F: Float>(t14292: F, t475: F, t503: F, t15092: F, t551: F, t554: F, t15093: F, t1607: F, t4349: F, t14608: F, t1553: F, t14607: F, t524: F, t547: F, t10334: F, t195: F, t217: F) -> (F, F, F, F, F, F, F) {
    let t41218 = t475 / t14292 / t503;
    let t41849 = t551 / t15092 / t554;
    let t41861 = t1607 * t15093;
    let t42126 = t4349 * t4349;
    let t42127 = 1.0 / t42126;
    let t42942 = t1553 * t14608;
    let t42957 = t524 / t14607 / t547;
    let t43141 = t195 / t10334 / t217;
    (t41218, t41849, t41861, t42127, t42942, t42957, t43141)
}
