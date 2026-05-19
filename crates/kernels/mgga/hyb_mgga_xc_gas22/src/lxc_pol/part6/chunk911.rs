//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 911/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk911<F: Float>(t1193: F, t6004: F, t3029: F, t60: F, t63: F, t66: F, t69: F, t1196: F, t1198: F, t1200: F, t1202: F, t1204: F, t1206: F, t1208: F, t1880: F, t1913: F, t3037: F, t3042: F, t3047: F, t3052: F, t584: F) -> (F, F) {
    let t8036 = t6004 * t1193;
    let t8041 = t60 * t3029;
    let t8046 = t63 * t3029;
    let t8051 = t66 * t3029;
    let t8056 = t69 * t3029;
    let t8059 = -F::new(2.0) / F::new(3.0) * t1196 * t1880 + t1198 * t1880 / F::new(8.0) - t1200 * t1880 / F::new(80.0) + t1202 * t1880 / F::new(1152.0) - t1204 * t1880 / F::new(21504.0) + t1206 * t1880 / F::new(491520.0) - t1208 * t1880 / F::cast_from(13271040.0_f64) + t8036 * t1880 / F::cast_from(412876800.0_f64) + t3037 * t1913 / F::new(640.0) - t8041 * t584 / F::new(5760.0) - t3042 * t1913 / F::new(11520.0) + t8046 * t584 / F::new(129024.0) + t3047 * t1913 / F::new(258048.0) - t8051 * t584 / F::new(3440640.0) - t3052 * t1913 / F::new(6881280.0) + t8056 * t584 / F::new(0.10616832e9);
    (t8036, t8059)
}
