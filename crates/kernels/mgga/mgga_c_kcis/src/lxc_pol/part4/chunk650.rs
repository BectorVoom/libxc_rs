//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 650/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk650<F: Float>(t1268: F, t3616: F, t1267: F, t426: F, t1236: F, t1239: F, t1240: F, t1269: F, t2818: F, t2823: F, t2827: F, t2832: F, t2848: F, t2853: F, t2858: F, t2862: F, t3052: F, t3172: F, t3174: F, t3180: F) -> (F, F, F, F, F, F, F) {
    let t3617 = t3616 * t1268;
    let t3620 = t1267 * t1267;
    let t3621 = t426 * t426;
    let t3622 = F::new(1.0) / t3621;
    let t3623 = t3620 * t3622;
    let t3638 = t1236 * t1239;
    let t3641 = -F::new(0.66725e-1) * t1240 * t3617 + F::new(0.66725e-1) * t1240 * t3623 - F::new(0.23214722222222222222e-2) * t2818 + F::new(0.15476481481481481481e-2) * t2823 + F::new(0.23214722222222222222e-2) * t2827 + F::new(0.11607361111111111111e-2) * t2832 + F::new(0.19345601851851851852e-2) * t2848 - F::new(0.23214722222222222222e-2) * t2853 - F::new(0.61905925925925925925e-2) * t2858 - F::new(0.23214722222222222222e-2) * t2862 + F::new(0.23214722222222222222e-2) * t3052 + F::new(0.17411041666666666666e-2) * t3172 + F::new(0.15476481481481481481e-2) * t3174 - F::new(0.34822083333333333332e-2) * t3180 - F::new(0.13345e0) * t3638 * t1269;
    (t3617, t3620, t3621, t3622, t3623, t3638, t3641)
}
