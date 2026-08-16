//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 760/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk760<F: Float>(t7: F, t143: F, t1270: F, t1285: F, t172: F, t187: F, t4045: F, t4046: F, t4082: F, t139: F, t214: F, t26: F, t3804: F, t2170: F, t3814: F, t776: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t144 = F::cast_from(0.135e1_f64) <= t143;
    let t4086 = piecewise3::<F>(t144, t4045, -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4046 * t187 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t1270 * t1285 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t172 * t4082);
    let t4087 = t139 * t4086;
    let t4088 = t4087 * t214;
    let t4089 = t26 * t4088;
    let t4094 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t3804);
    let t4104 = piecewise3::<F>(t8, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2170 * t3814 - t776 * t3804 / F::cast_from(3.0_f64));
    (t4086, t4088, t4089, t4094, t4104)
}
