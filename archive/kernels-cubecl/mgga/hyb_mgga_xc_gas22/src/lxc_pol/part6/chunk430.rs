//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 430/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk430<F: Float>(t1880: F, t1913: F, t1941: F, t51: F, t54: F, t565: F, t57: F, t588: F, t592: F, t596: F, t60: F, t600: F, t604: F, t608: F, t612: F, t63: F, t66: F, t69: F) -> F {
    let t1946 = t51 * t1880 / F::cast_from(6.0_f64) - t565 * t1913 / F::cast_from(18.0_f64) - t54 * t1880 / F::cast_from(48.0_f64) + t588 * t1913 / F::cast_from(240.0_f64) + t57 * t1880 / F::cast_from(640.0_f64) - t592 * t1913 / F::cast_from(4480.0_f64) - t60 * t1880 / F::cast_from(11520.0_f64) + t596 * t1913 / F::cast_from(103680.0_f64) + t63 * t1880 / F::cast_from(258048.0_f64) - t600 * t1913 / F::cast_from(2838528.0_f64) - t66 * t1880 / F::cast_from(6881280.0_f64) + t604 * t1913 / F::cast_from(89456640.0_f64) + t69 * t1880 / F::cast_from(0.21233664e9_f64) - t608 * t1913 / F::cast_from(0.31850496e10_f64) - t1941 * t1880 / F::cast_from(0.74317824e10_f64) + t612 * t1913 / F::cast_from(0.1263403008e12_f64);
    t1946
}
