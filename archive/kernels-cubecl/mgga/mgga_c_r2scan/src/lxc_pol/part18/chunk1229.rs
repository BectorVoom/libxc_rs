//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1229/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1229<F: Float>(t10760: F, t29783: F, t6093: F, t3591: F, t39739: F, t38144: F, t40223: F, t40233: F, t40234: F, t41753: F, t43654: F, t43657: F, t43660: F, t43664: F, t43667: F) -> F {
    let t43670 = t6093 * t10760 * t29783;
    let t43672 = t39739 * t3591;
    let t43674 = -F::cast_from(0.65854491829355115988e0_f64) * t43654 + F::cast_from(0.32927245914677557994e0_f64) * t43657 - t40223 - F::cast_from(0.23287303101564395623e-1_f64) * t43660 + t41753 + t40233 + F::cast_from(0.58544643236296698111e-1_f64) * t40234 + F::cast_from(0.43663693315433241792e-2_f64) * t43664 - F::cast_from(0.21831846657716620896e-2_f64) * t43667 - F::cast_from(0.65495539973149862688e-2_f64) * t43670 - t38144 - F::cast_from(0.86682217400542685632e-1_f64) * t43672;
    t43674
}
