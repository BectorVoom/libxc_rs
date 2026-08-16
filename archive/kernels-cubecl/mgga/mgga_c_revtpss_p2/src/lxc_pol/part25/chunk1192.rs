//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1192/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1192<F: Float>(t26034: F, t545: F, t2028: F, t3920: F, t7246: F, t2023: F, t2453: F, t3908: F, t2022: F, t3923: F, t543: F, t7301: F) -> (F, F, F, F, F, F, F) {
    let t26035 = t545 * t26034;
    let t26036 = t2028 * t26035;
    let t26040 = F::cast_from(0.13009920719177044025e-1_f64) * t7246 * t3920;
    let t26041 = t2453 * t2023;
    let t26043 = F::cast_from(0.11565819519348392139e-2_f64) * t26041 * t3908;
    let t26044 = t2022 * t3923;
    let t26045 = t26044 * t543;
    let t26046 = t7301 * t26045;
    (t26035, t26036, t26040, t26041, t26043, t26044, t26046)
}
