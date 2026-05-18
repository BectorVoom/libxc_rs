//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 693/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk693<F: Float>(t5: F, t72: F, t7714: F, t1927: F, t1493: F, t76: F, t1926: F, t1923: F, t1928: F, t6958: F, t7702: F, t7706: F, t7709: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t7715 = t7714 * t72;
    let t7716 = t7715 * t1927;
    let t7719 = t76 * t1493;
    let t7720 = t1926 * t7719;
    let t7724 = piecewise3::<f64>(t8, F::new(0.0), -t7702 * t1928 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t6958 * t7706 + t7709 * t1928 / F::new(3.0) - t1923 * t7716 / F::new(6.0) - t1923 * t7720 / F::new(6.0));
    (t7715, t7716, t7719, t7720, t7724)
}
