//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1229/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1229<F: Float>(t10760: F, t29783: F, t6093: F, t3591: F, t39739: F, t38144: F, t40223: F, t40233: F, t40234: F, t41753: F, t43654: F, t43657: F, t43660: F, t43664: F, t43667: F) -> F {
    let t43670 = t6093 * t10760 * t29783;
    let t43672 = t39739 * t3591;
    let t43674 = -F::new(0.65854491829355115988e0) * t43654 + F::new(0.32927245914677557994e0) * t43657 - t40223 - F::new(0.23287303101564395623e-1) * t43660 + t41753 + t40233 + F::new(0.58544643236296698111e-1) * t40234 + F::new(0.43663693315433241792e-2) * t43664 - F::new(0.21831846657716620896e-2) * t43667 - F::new(0.65495539973149862688e-2) * t43670 - t38144 - F::new(0.86682217400542685632e-1) * t43672;
    t43674
}
