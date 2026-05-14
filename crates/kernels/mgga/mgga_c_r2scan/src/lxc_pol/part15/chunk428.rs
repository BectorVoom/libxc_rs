//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 428/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk428<F: Float>(t188: F, t1893: F, t1647: F, t1891: F, t644: F, t652: F, t621: F, t650: F, t1800: F, t190: F, t632: F, t175: F, t648: F, t14: F, t653: F, t645: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1894 = t188 * t1893;
    let t1897 = 0.51726012919273400301e3 * t1891 * t1894 * t1647;
    let t1898 = t644 * t652;
    let t1901 = 0.32163958997385070134e2 * t650 * t1898 * t621;
    let t1904 = 2.0 * t632 * t190 * t1800;
    let t1906 = 1.0 / t648 / t175;
    let t1907 = t14 * t1906;
    let t1910 = 0.96491876992155210402e2 * t1907 * t653 * t1647;
    let t1913 = 4.0 * t632 * t645 * t621;
    (t1894, t1897, t1898, t1901, t1904, t1906, t1907, t1910, t1913)
}
