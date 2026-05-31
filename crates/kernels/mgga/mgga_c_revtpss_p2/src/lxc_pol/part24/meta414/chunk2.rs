//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1359/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1359<F: Float>(t342: F, t42859: F, t11626: F, t358: F, t3145: F, t365: F, t360: F, t3153: F) -> (F, F, F, F, F, F) {
    let t42860 = t342 * t42859;
    let t42862 = F::cast_from(1.0_f64) / t11626 / t358;
    let t42864 = t3145 * t3145;
    let t42865 = F::cast_from(1.0_f64) / t42864;
    let t42866 = t365 * t42865;
    let t42868 = t42860 * t42862 * t360 * t42866;
    let t42871 = t3153 * t3153;
    (t42860, t42862, t42865, t42866, t42868, t42871)
}
