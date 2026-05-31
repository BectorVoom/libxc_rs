//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1190/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1190<F: Float>(t12295: F, t12351: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t12344: F, t12347: F, t12354: F) -> F {
    let t12459 = F::cast_from(0.16068111111111111111e1_f64) * t12295;
    let t12460 = F::cast_from(0.46308888888888888888e0_f64) * t12351;
    let t12463 = F::cast_from(0.34431666666666666666e0_f64) * t12299 + F::cast_from(0.57386111111111111112e0_f64) * t12307 + F::cast_from(0.68863333333333333332e0_f64) * t12297 - F::cast_from(0.103295e1_f64) * t12301 - F::cast_from(0.51647499999999999999e0_f64) * t12303 - F::cast_from(0.20659e1_f64) * t12310 + F::cast_from(0.309885e1_f64) * t12314 + F::cast_from(0.516475e0_f64) * t12320 - F::cast_from(0.52945875e1_f64) * t12344 + F::cast_from(0.94674375e0_f64) * t12347 - t12459 - t12460 + F::cast_from(0.309885e1_f64) * t12317 + F::cast_from(0.6311625e0_f64) * t12354;
    t12463
}
