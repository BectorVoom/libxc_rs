//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 321/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk321<F: Float>(t1306: F, t1891: F, t469: F, t1305: F) -> (F, F, F) {
    let t1893 = -t1306 - F::cast_from(0.17808333333333333333e-1_f64) * t1891;
    let t1895 = F::cast_from(0.62182e-1_f64) * t1893 * t469;
    let t1897 = -t1305 / F::cast_from(3.0_f64) - t1891 / F::cast_from(3.0_f64);
    (t1893, t1895, t1897)
}
