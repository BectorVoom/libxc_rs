//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 967/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk967<F: Float>(t11013: F, t2304: F, t875: F, t3434: F, t3439: F, t106: F, t1550: F, t97: F, t3271: F, t10918: F, t3262: F, t3264: F) -> (F, F, F, F, F, F) {
    let t11014 = F::new(3.0) / F::new(2.0) * t11013;
    let t11015 = t2304 * t875;
    let t11017 = t3434 * t11015 * t3439;
    let t11018 = F::new(0.1951603679568577289e-3) * t11017;
    let t11020 = t97 * t106 * t1550;
    let t11021 = t11020 * t3271;
    let t11022 = t11021 / F::new(4.0);
    let t11024 = t3262 * t10918 * t3264;
    (t11014, t11015, t11018, t11020, t11022, t11024)
}
