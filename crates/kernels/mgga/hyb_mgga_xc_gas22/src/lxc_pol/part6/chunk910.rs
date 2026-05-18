//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 910/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk910<F: Float>(t43: F, t7983: F, t1941: F, t3029: F, t51: F, t54: F, t57: F, t1913: F, t3002: F, t3032: F, t3057: F, t3062: F, t565: F, t584: F, t588: F, t592: F, t596: F, t600: F, t604: F, t608: F, t612: F) -> (F, F) {
    let t45 = F::new(0.135e1) < t43;
    let t7984 = piecewise3::<f64>(t45, t7983, F::new(0.0));
    let t8003 = t1941 * t3029;
    let t8008 = t51 * t3029;
    let t8013 = t54 * t3029;
    let t8018 = t57 * t3029;
    let t8021 = t596 * t7984 / F::new(103680.0) - t600 * t7984 / F::new(2838528.0) + t604 * t7984 / F::new(89456640.0) - t608 * t7984 / F::new(0.31850496e10) + t612 * t7984 / F::new(0.1263403008e12) - t565 * t7984 / F::new(18.0) + t588 * t7984 / F::new(240.0) - t592 * t7984 / F::new(4480.0) + t3057 * t1913 / F::new(0.21233664e9) - t8003 * t584 / F::new(0.37158912e10) - t3062 * t1913 / F::new(0.74317824e10) + t8008 * t584 / F::new(3.0) + t3002 * t1913 / F::new(6.0) - t8013 * t584 / F::new(24.0) - t3032 * t1913 / F::new(48.0) + t8018 * t584 / F::new(320.0);
    (t7984, t8021)
}
