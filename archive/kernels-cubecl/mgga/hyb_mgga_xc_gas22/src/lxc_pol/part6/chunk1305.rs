//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1305/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1305<F: Float>(t3188: F, t708: F, t10267: F, t10270: F, t10275: F, t10278: F, t10331: F, t1255: F, t1257: F, t1259: F, t1261: F, t1263: F, t1265: F, t1267: F, t151: F, t163: F, t166: F, t169: F, t2070: F, t3201: F, t3206: F, t8267: F, t8335: F) -> F {
    let t28439 = t708 * t3188;
    let t28456 = t10267 * t2070 / F::cast_from(640.0_f64) + t10270 * t2070 / F::cast_from(1152.0_f64) - t3201 * t8335 / F::cast_from(5760.0_f64) - t10275 * t2070 / F::cast_from(11520.0_f64) - t10278 * t2070 / F::cast_from(21504.0_f64) + t3206 * t8335 / F::cast_from(129024.0_f64) + t151 * t10331 * t708 / F::cast_from(3.0_f64) + t163 * t10331 * t708 / F::cast_from(129024.0_f64) - t166 * t10331 * t708 / F::cast_from(3440640.0_f64) + t169 * t10331 * t708 / F::cast_from(0.10616832e9_f64) + t1265 * t28439 / F::cast_from(122880.0_f64) - t1267 * t28439 / F::cast_from(3317760.0_f64) + t8267 * t28439 / F::cast_from(103219200.0_f64) - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1255 * t28439 + t1257 * t28439 / F::cast_from(2.0_f64) - t1259 * t28439 / F::cast_from(20.0_f64) + t1261 * t28439 / F::cast_from(288.0_f64) - t1263 * t28439 / F::cast_from(5376.0_f64);
    t28456
}
