//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 357/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk357<F: Float>(t143: F, t1252: F, t1255: F, t1257: F, t1259: F, t1261: F, t1263: F, t1265: F, t1267: F, t1270: F, t1285: F, t172: F, t187: F, t694: F) -> F {
    let t144 = F::new(0.135e1) <= t143;
    let t1289 = piecewise3::<F>(t144, -t694 * t1252 / F::new(18.0) + t1255 / F::new(240.0) - t1257 / F::new(4480.0) + t1259 / F::new(103680.0) - t1261 / F::new(2838528.0) + t1263 / F::cast_from(89456640.0_f64) - t1265 / F::new(0.31850496e10) + t1267 / F::cast_from(0.1263403008e12_f64), -F::new(8.0) / F::new(3.0) * t1270 * t187 - F::new(8.0) / F::new(3.0) * t172 * t1285);
    t1289
}
