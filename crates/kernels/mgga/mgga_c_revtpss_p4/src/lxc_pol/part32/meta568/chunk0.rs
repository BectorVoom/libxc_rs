//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1892/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1892<F: Float>(t101788: F, t6960: F, t26205: F, t7709: F, t28640: F, t6963: F, t28141: F, t7349: F, t101226: F, t2047: F, t7706: F, t95283: F) -> (F, F, F, F, F, F) {
    let t101790 = F::new(80.0) / F::new(9.0) * t101788 * t6960;
    let t101793 = t7709 * t26205;
    let t101811 = F::new(32.0) / F::new(9.0) * t6963 * t28640;
    let t101820 = F::new(32.0) / F::new(9.0) * t28141 * t7349;
    let t101850 = t2047 * t101226;
    let t101870 = F::new(80.0) / F::new(9.0) * t95283 * t7706;
    (t101790, t101793, t101811, t101820, t101850, t101870)
}
