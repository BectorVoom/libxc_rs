//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1075/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1075<F: Float>(t12092: F, t12056: F, t12057: F, t12060: F, t12061: F, t12067: F, t12071: F, t12078: F, t12082: F, t12086: F, t12088: F, t2277: F, t6718: F, t9669: F) -> (F, F) {
    let t12093 = F::new(7.0) / F::new(72.0) * t12092;
    let t12094 = -t12056 + F::new(7.0) / F::new(2304.0) * t12057 + F::new(119.0) / F::new(3456.0) * t9669 + t12060 - F::new(7.0) / F::new(2304.0) * t12061 - t12067 - t12071 + t12078 + t12082 - t12086 - t2277 * t12088 / F::new(1536.0) + F::new(119.0) / F::new(6912.0) * t6718 - t12093;
    (t12093, t12094)
}
