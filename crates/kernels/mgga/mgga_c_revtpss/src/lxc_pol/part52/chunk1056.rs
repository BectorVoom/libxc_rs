//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1056/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1056<F: Float>(t5: F, t32597: F, t8623: F, t1925: F, t84: F, t640: F, t8621: F, t32151: F, t32581: F, t32584: F, t32586: F, t32590: F, t32593: F, t8620: F) -> (F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t32599 = F::new(5.0) / F::new(27.0) * t32597 * t8623;
    let t32600 = t84 * t1925;
    let t32602 = t8621 * t32600 * t640;
    let t32608 = piecewise3::<f64>(t8, F::new(0.0), -F::new(5.0) / F::new(72.0) * t32581 * t8623 + F::new(5.0) / F::new(12.0) * t32584 * t32586 + F::new(5.0) / F::new(18.0) * t32590 * t32593 + t32599 - F::new(5.0) / F::new(36.0) * t8620 * t32602 - F::new(5.0) / F::new(72.0) * t8620 * t32151);
    (t32599, t32600, t32602, t32608)
}
