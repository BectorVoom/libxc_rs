//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1002/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1002<F: Float>(t174: F, t740: F, t9323: F, t447: F, t2001: F, t4134: F, t1610: F, t2104: F, t2153: F, t2539: F, t9275: F, t2146: F, t2537: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t18374 = F::new(2.0) * t740;
    let t18375 = F::new(6.0) * t9323;
    let t18376 = -t18374 + t18375;
    let t18377 = piecewise3::<F>(t175, F::new(0.0), t18376);
    let t18378 = t447 * t18377;
    let t20905 = t4134 * t2001;
    let t23096 = t2104 * t1610;
    let t26390 = t2153 * t2539;
    let t26391 = t9275 * t26390;
    let t26392 = F::new(6.0) * t26391;
    let t26398 = t2146 * t2537;
    (t18377, t18378, t20905, t23096, t26390, t26391, t26392, t26398)
}
