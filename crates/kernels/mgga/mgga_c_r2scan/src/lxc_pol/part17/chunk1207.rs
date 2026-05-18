//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1207/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1207<F: Float>(t3275: F, t3277: F, t44040: F, t11550: F, t12056: F, t3262: F, t11189: F, t43979: F, t3579: F, t41327: F, t39010: F, t42472: F) -> (F, F, F, F, F) {
    let t44043 = F::new(5.0) / F::new(16.0) * t3275 * t44040 * t3277;
    let t44046 = F::new(3.0) / F::new(2.0) * t3262 * t12056 * t11550;
    let t44049 = F::new(45.0) / F::new(32.0) * t3275 * t11189 * t43979;
    let t44051 = F::new(5.0) / F::new(8.0) * t3579 * t41327;
    let t44054 = F::new(585.0) / F::new(256.0) * t3275 * t39010 * t42472;
    (t44043, t44046, t44049, t44051, t44054)
}
