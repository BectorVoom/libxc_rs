//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2087/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2087<F: Float>(t3201: F, t7801: F, t1058: F, t27467: F, t15775: F, t7132: F, t100054: F, t3299: F, t4857: F, t7125: F, t25495: F, t4845: F) -> (F, F, F, F, F, F) {
    let t100272 = t7801 * t3201;
    let t100275 = F::cast_from(0.57165357490759649296e-3_f64) * t27467 * t1058;
    let t100289 = F::cast_from(0.6351706387862183255e-3_f64) * t7132 * t15775;
    let t100302 = t3299 * t100054;
    let t100324 = t4857 * t7125;
    let t100327 = t25495 * t4845;
    (t100272, t100275, t100289, t100302, t100324, t100327)
}
