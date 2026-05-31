//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1613/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1613<F: Float>(t3362: F, t414: F, t66: F, t42859: F, t460: F, t42865: F, t479: F, t1244: F, t3601: F, t482: F, t42871: F, t471: F) -> (F, F, F, F, F, F, F) {
    let t44361 = F::cast_from(1.0_f64) / t414 / t3362;
    let t44362 = t66 * t44361;
    let t44372 = t460 * t42859;
    let t44373 = t479 * t42865;
    let t44375 = t44372 * t1244 * t44373;
    let t44376 = t3601 * t3601;
    let t44377 = t482 * t44376;
    let t44378 = t42871 * t471;
    (t44362, t44372, t44373, t44375, t44376, t44377, t44378)
}
