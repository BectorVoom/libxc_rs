//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1142/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1142<F: Float>(t102204: F, t94589: F, t2470: F, t28779: F, t25895: F, t94771: F, t2435: F, t28902: F, t2453: F, t3908: F, t8086: F, t25878: F) -> (F, F, F, F, F, F) {
    let t102205 = t94589 * t102204;
    let t102218 = t28779 * t2470;
    let t102219 = t25895 * t102218;
    let t102225 = t94771 * t102204;
    let t102249 = t2435 * t28902;
    let t102266 = t2453 * t8086 * t3908;
    let t102293 = t25878 * t102218;
    (t102205, t102219, t102225, t102249, t102266, t102293)
}
