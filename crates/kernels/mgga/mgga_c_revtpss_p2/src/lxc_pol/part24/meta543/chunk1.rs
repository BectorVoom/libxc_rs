//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1604/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1604<F: Float>(t87280: F, t87292: F, t162: F, t187: F, t150: F, t190: F, t18850: F, t2403: F, t39419: F, t39422: F, t39429: F, t39432: F, t39442: F, t5962: F, t87262: F, t87263: F, t87265: F, t87267: F, t87268: F) -> (F, F, F) {
    let t87293 = t87280 + t87292;
    let t87296 = F::cast_from(0.19751673498613801407e-1_f64) * t87293 * t162 * t187;
    let t87298 = t150 * t87293 * t190;
    let t87302 = F::new(18.0) * t18850 * t2403 * t5962 - t39419 - t39422 - t39429 - t39432 + t39442 + t87262 + t87263 + t87265 + t87267 - t87268 + t87296 + t87298;
    (t87296, t87298, t87302)
}
