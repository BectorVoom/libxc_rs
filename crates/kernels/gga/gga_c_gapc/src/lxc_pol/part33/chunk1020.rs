//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1020/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1020<F: Float>(t137: F, t3074: F, t34509: F, t5126: F, t26578: F, t34503: F, t203: F, t27596: F, t5698: F, t6: F, t1030: F, t144: F, t33521: F, t4052: F, t34515: F, t34517: F, t34520: F, t34522: F, t34525: F, t34528: F, t34530: F, t34533: F) -> (F, F) {
    let t34535 = t3074 * t137;
    let t34537 = t34509 * t34535 * t5126;
    let t34539 = t34503 * t26578;
    let t34546 = t5698 * t203 * t6 * t27596;
    let t34547 = t1030 * t4052 * t33521 * t144 * t34546;
    let t34549 = 0.25301920572916666668e-5 * t34515 + 0.12650960286458333334e-5 * t34517 + 0.25301920572916666668e-5 * t34520 + 0.12650960286458333334e-5 * t34522 - 0.25301920572916666668e-5 * t34525 - 0.24458523220486111112e-4 * t34528 + 0.2845640240200497334e-7 * t34530 + 0.34380927311705569432e-8 * t34533 - 0.65555167711046006955e-8 * t34537 + 0.70344136651018351214e-8 * t34539 + 0.28199579487947481489e-8 * t34547;
    (t34535, t34549)
}
