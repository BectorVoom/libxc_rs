//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1272/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1272<F: Float>(t43: F, t27288: F, t27308: F, t27348: F, t27403: F, t3029: F, t1941: F, t3002: F, t51: F, t54: F, t565: F, t57: F, t584: F, t588: F, t592: F, t596: F, t60: F, t600: F, t604: F, t608: F, t612: F, t63: F, t66: F, t69: F, t7984: F, t9915: F) -> (F, F) {
    let t45 = F::new(0.135e1) < t43;
    let t27405 = t27288 + t27308 + t27348 + t27403;
    let t27406 = piecewise3::<F>(t45, t27405, F::new(0.0));
    let t27423 = t3029 * t3029;
    let t27440 = t3002 * t7984 / F::new(3.0) + t57 * t9915 * t584 / F::new(320.0) - t592 * t27406 / F::new(4480.0) + t596 * t27406 / F::new(103680.0) - t600 * t27406 / F::new(2838528.0) + t604 * t27406 / F::cast_from(89456640.0_f64) - t608 * t27406 / F::new(0.31850496e10) + t612 * t27406 / F::cast_from(0.1263403008e12_f64) - t565 * t27406 / F::new(18.0) + t588 * t27406 / F::new(240.0) - t54 * t27423 / F::new(24.0) + t57 * t27423 / F::new(320.0) - t60 * t27423 / F::new(5760.0) + t63 * t27423 / F::new(129024.0) - t66 * t27423 / F::new(3440640.0) + t69 * t27423 / F::new(0.10616832e9) - t1941 * t27423 / F::new(0.37158912e10) + t51 * t27423 / F::new(3.0);
    (t27405, t27440)
}
