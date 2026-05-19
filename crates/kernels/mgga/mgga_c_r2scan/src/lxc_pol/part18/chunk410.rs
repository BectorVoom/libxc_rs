//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 410/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk410<F: Float>(t188: F, t1893: F, t1647: F, t1891: F, t644: F, t652: F, t621: F, t650: F, t1800: F, t190: F, t632: F, t175: F, t648: F) -> (F, F, F, F, F, F) {
    let t1894 = t188 * t1893;
    let t1897 = F::cast_from(0.51726012919273400301e3_f64) * t1891 * t1894 * t1647;
    let t1898 = t644 * t652;
    let t1901 = F::cast_from(0.32163958997385070134e2_f64) * t650 * t1898 * t621;
    let t1904 = F::new(2.0) * t632 * t190 * t1800;
    let t1906 = F::new(1.0) / t648 / t175;
    (t1894, t1897, t1898, t1901, t1904, t1906)
}
