//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1128/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1128<F: Float>(t11618: F, t11623: F, t11631: F, t11634: F, t11637: F, t12020: F, t11858: F, t39464: F, t39470: F, t39485: F, t39558: F, t39637: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41107 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t11618;
    let t41108 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t11623;
    let t41109 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t11631;
    let t41110 = t11634 / F::cast_from(2.0_f64);
    let t41111 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t11637;
    let t41112 = F::cast_from(2.0_f64) * t12020;
    let t41113 = t11858 / F::cast_from(2.0_f64);
    let t41395 = F::cast_from(0.11902492299418487743e0_f64) * t39464;
    let t41397 = F::cast_from(0.28914548798370980346e-3_f64) * t39470;
    let t41405 = F::cast_from(0.93443229163669953711e-1_f64) * t39485;
    let t41439 = F::cast_from(0.45022119329691164871e0_f64) * t39558;
    let t41478 = F::cast_from(0.32927245914677557993e-1_f64) * t39637;
    (t41107, t41108, t41109, t41110, t41111, t41112, t41113, t41395, t41397, t41405, t41439, t41478)
}
