//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1269/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1269<F: Float>(t1890: F, t9879: F, t9874: F, t7942: F, t9883: F, t9906: F, t9901: F, t9890: F, t9894: F, t23313: F, t23315: F, t23317: F, t23319: F, t23321: F) -> F {
    let t27294 = t1890 * t9879;
    let t27296 = t1890 * t9874;
    let t27298 = t7942 * t9883;
    let t27300 = t1890 * t9906;
    let t27302 = t1890 * t9901;
    let t27304 = t1890 * t9890;
    let t27306 = t7942 * t9894;
    let t27308 = F::new(10.0) / F::new(729.0) * t23313 + F::new(8.0) / F::new(243.0) * t23315 - F::new(2.0) / F::new(81.0) * t23317 - F::new(8.0) / F::new(81.0) * t23319 + F::new(16.0) / F::new(243.0) * t23321 - F::new(4.0) / F::new(81.0) * t27294 + F::new(10.0) / F::new(729.0) * t27296 + F::new(44.0) / F::new(243.0) * t27298 - F::new(2.0) / F::new(81.0) * t27300 + F::new(2.0) / F::new(243.0) * t27302 + F::new(2.0) / F::new(27.0) * t27304 - F::new(44.0) / F::new(81.0) * t27306;
    t27308
}
