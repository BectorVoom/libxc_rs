//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 507/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk507<F: Float>(t225: F, t3999: F, t213: F, t4086: F, t1892: F, t545: F, t869: F, t689: F, t72: F, t1432: F, t686: F, t1385: F, t116: F, t1518: F, t2219: F, t2221: F, t2223: F, t2226: F, t2228: F, t2230: F, t2233: F, t2235: F, t2239: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5744 = t225 * t3999;
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    let t5759 = t545 * t1892;
    let t5760 = t869 * t5759;
    let t5761 = t689 * t5760;
    let t5763 = t1892 * t72;
    let t5765 = t1432 * t5763 * t686;
    let t5767 = t1385 * t1892;
    let t5801 = t116 * t1518;
    let t5812 = t2219 + t2221 + t2223 + t2226 + t2228 + t2230 + t2233 + t2235 + t2239;
    (t5744, t5745, t5755, t5759, t5760, t5761, t5763, t5765, t5767, t5801, t5812)
}
