//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 795/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk795<F: Float>(t245: F, t24250: F, t24856: F, t1459: F, t1580: F, t21: F, t363: F, t5: F, t6200: F, t2405: F, t6273: F, t10479: F, t1501: F, t668: F) -> (F, F, F, F, F) {
    let t246 = 10000000.0 <= t245;
    let t24857 = t24250 + t24856;
    let t24868 = piecewise3(t246, 0.0, t5 * t24857 * t21 / 4.0 + t5 * t6200 * t363 / 2.0 + t5 * t1459 * t1580 / 4.0);
    let t24869 = t6273 * t2405;
    let t24870 = t10479 * t24869;
    let t24873 = t1501 * t668;
    (t24857, t24868, t24869, t24870, t24873)
}
