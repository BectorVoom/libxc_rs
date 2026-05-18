//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1206/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1206<F: Float>(t11342: F, t43726: F, t11555: F, t12098: F, t3275: F, t11486: F, t3262: F, t11506: F, t41337: F, t3579: F, t41816: F, t12811: F, t1561: F) -> (F, F, F, F, F, F) {
    let t44029 = F::new(3.0) / F::new(4.0) * t43726 * t11342;
    let t44032 = F::new(5.0) / F::new(8.0) * t3275 * t12098 * t11555;
    let t44035 = F::new(15.0) / F::new(8.0) * t3262 * t12098 * t11486;
    let t44037 = F::new(3.0) / F::new(2.0) * t11506 * t41337;
    let t44039 = F::new(5.0) / F::new(8.0) * t3579 * t41816;
    let t44040 = t1561 * t12811;
    (t44029, t44032, t44035, t44037, t44039, t44040)
}
