//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 349/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk349<F: Float>(t43: F, t1193: F, t1196: F, t1198: F, t1200: F, t1202: F, t1204: F, t1206: F, t1208: F, t1211: F, t1226: F, t565: F, t72: F, t88: F) -> F {
    let t44 = F::new(0.135e1) <= t43;
    let t1230 = piecewise3::<F>(t44, -t565 * t1193 / F::new(18.0) + t1196 / F::new(240.0) - t1198 / F::new(4480.0) + t1200 / F::new(103680.0) - t1202 / F::new(2838528.0) + t1204 / F::cast_from(89456640.0_f64) - t1206 / F::new(0.31850496e10) + t1208 / F::cast_from(0.1263403008e12_f64), -F::new(8.0) / F::new(3.0) * t1211 * t88 - F::new(8.0) / F::new(3.0) * t72 * t1226);
    t1230
}
