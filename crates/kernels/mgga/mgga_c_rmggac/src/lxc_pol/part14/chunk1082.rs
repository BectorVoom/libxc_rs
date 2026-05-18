//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1082/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1082<F: Float>(t42296: F, t42297: F, t42298: F, t42299: F, t42300: F, t42301: F, t7886: F, t8197: F, t9501: F, t9600: F, t9601: F, t9035: F) -> (F, F) {
    let t42302 = -t8197 + t9501 + t42296 + t7886 - t42297 + t42298 + t9600 + t9601 + t42299 + t42300 - t42301;
    let t42306 = F::new(0.11974241701863808564e0) * t9035;
    (t42302, t42306)
}
