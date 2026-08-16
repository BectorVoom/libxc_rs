//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1229/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1229(t10760: f64, t29783: f64, t6093: f64, t3591: f64, t39739: f64, t38144: f64, t40223: f64, t40233: f64, t40234: f64, t41753: f64, t43654: f64, t43657: f64, t43660: f64, t43664: f64, t43667: f64) -> f64 {
    let t43670 = t6093 * t10760 * t29783;
    let t43672 = t39739 * t3591;
    let t43674 = -0.65854491829355115988e0_f64 * t43654 + 0.32927245914677557994e0_f64 * t43657 - t40223 - 0.23287303101564395623e-1_f64 * t43660 + t41753 + t40233 + 0.58544643236296698111e-1_f64 * t40234 + 0.43663693315433241792e-2_f64 * t43664 - 0.21831846657716620896e-2_f64 * t43667 - 0.65495539973149862688e-2_f64 * t43670 - t38144 - 0.86682217400542685632e-1_f64 * t43672;
    t43674
}
