//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 502/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk502<F: Float>(t118: F, t1502: F, t1519: F, t1843: F, t1847: F, t1911: F, t508: F, t511: F, t569: F, t651: F, t3: F, t117: F, t1518: F, t572: F, t573: F, t76: F, t84: F) -> (F, F, F, F, F, F) {
    let t1913 = -t118 * t1843 - t1502 * t508 - 2.0 * t1519 * t651 + t1847 * t569 + t1911 * t511;
    let t1914 = t3 * t1913;
    let t1916 = param_d * t1913;
    let t1918 = t117 * t1518;
    let t1921 = t1916 * t573 + 3.0 * t1918 * t572;
    let t1927 = t76 * t84;
    (t1913, t1914, t1916, t1918, t1921, t1927)
}
