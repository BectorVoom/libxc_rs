//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 926/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk926<F: Float>(t143: F, t8334: F, t1255: F, t1257: F, t1259: F, t1261: F, t1263: F, t1265: F, t1267: F, t2042: F, t694: F, t712: F, t716: F, t720: F, t724: F, t728: F, t732: F, t736: F, t8267: F) -> (F, F) {
    let t145 = F::new(0.135e1) < t143;
    let t8335 = piecewise3::<F>(t145, t8334, F::new(0.0));
    let t8352 = t8267 * t2042 / F::cast_from(412876800.0_f64) - F::new(2.0) / F::new(3.0) * t1255 * t2042 + t1257 * t2042 / F::new(8.0) - t1259 * t2042 / F::new(80.0) + t1261 * t2042 / F::new(1152.0) - t1263 * t2042 / F::new(21504.0) + t1265 * t2042 / F::new(491520.0) - t1267 * t2042 / F::cast_from(13271040.0_f64) - t732 * t8335 / F::new(0.31850496e10) + t736 * t8335 / F::cast_from(0.1263403008e12_f64) - t694 * t8335 / F::new(18.0) + t712 * t8335 / F::new(240.0) - t716 * t8335 / F::new(4480.0) + t720 * t8335 / F::new(103680.0) - t724 * t8335 / F::new(2838528.0) + t728 * t8335 / F::cast_from(89456640.0_f64);
    (t8335, t8352)
}
