//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 773/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk773<F: Float>(t5: F, t265: F, t393: F, t1923: F, t2123: F, t7566: F, t7702: F, t7706: F, t7709: F, t8144: F, t8147: F, t117: F, t1518: F, t2163: F, t7855: F) -> (F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t394 = t265 < t393;
    let t8151 = piecewise3::<f64>(t8, F::new(0.0), -t7702 * t2123 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t7566 * t7706 + t7709 * t2123 / F::new(3.0) - t1923 * t8144 / F::new(6.0) - t1923 * t8147 / F::new(6.0));
    let t8152 = t8151 * t117;
    let t8158 = t2163 * t1518;
    let t8161 = piecewise3::<f64>(t394, F::new(0.0), t7855);
    (t8151, t8152, t8158, t8161)
}
