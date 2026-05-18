//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1273/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1273<F: Float>(t2042: F, t28956: F, t2113: F, t28271: F, t28277: F, t28974: F, t572: F, t7741: F, t26733: F, t1459: F, t34366: F, t28265: F) -> (F, F, F, F, F, F, F) {
    let t129080 = F::new(3.0) * t28956 * t2042;
    let t129082 = F::new(6.0) * t2113 * t28271;
    let t129084 = F::new(6.0) * t2113 * t28277;
    let t129089 = F::new(6.0) * t572 * t28974 * t7741;
    let t129092 = F::new(6.0) * t572 * t26733 * t7741;
    let t129095 = F::new(6.0) * t1459 * t34366;
    let t129097 = F::new(6.0) * t2113 * t28265;
    (t129080, t129082, t129084, t129089, t129092, t129095, t129097)
}
