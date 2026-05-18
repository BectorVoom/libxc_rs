//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1128/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1128<F: Float>(t11618: F, t11623: F, t11631: F, t11634: F, t11637: F, t12020: F, t11858: F, t39464: F, t39470: F, t39485: F, t39558: F, t39637: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41107 = F::new(5.0) / F::new(8.0) * t11618;
    let t41108 = F::new(45.0) / F::new(32.0) * t11623;
    let t41109 = F::new(5.0) / F::new(8.0) * t11631;
    let t41110 = t11634 / F::new(2.0);
    let t41111 = F::new(3.0) / F::new(2.0) * t11637;
    let t41112 = F::new(2.0) * t12020;
    let t41113 = t11858 / F::new(2.0);
    let t41395 = F::new(0.11902492299418487743e0) * t39464;
    let t41397 = F::new(0.28914548798370980346e-3) * t39470;
    let t41405 = F::new(0.93443229163669953711e-1) * t39485;
    let t41439 = F::new(0.45022119329691164871e0) * t39558;
    let t41478 = F::new(0.32927245914677557993e-1) * t39637;
    (t41107, t41108, t41109, t41110, t41111, t41112, t41113, t41395, t41397, t41405, t41439, t41478)
}
