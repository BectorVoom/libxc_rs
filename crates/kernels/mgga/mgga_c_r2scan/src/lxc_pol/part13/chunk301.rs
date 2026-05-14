//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 301/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk301<F: Float>(t322: F, t1020: F, t333: F, t335: F, t337: F, t339: F, t341: F, t1012: F) -> (F, F, F, F, F, F) {
    let t332 = 0.25e1 < t322;
    let t1022 = t333 * t1020;
    let t1024 = t335 * t1020;
    let t1026 = t337 * t1020;
    let t1028 = t339 * t1020;
    let t1030 = t341 * t1020;
    let t1035 = piecewise3(t332, t1012, 0.0);
    (t1022, t1024, t1026, t1028, t1030, t1035)
}
