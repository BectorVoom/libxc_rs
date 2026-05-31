//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1281/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1281<F: Float>(t18427: F, t18445: F, t18554: F, t18555: F, t27262: F, t27292: F, t27295: F, t31067: F, t31088: F, t31204: F, t31206: F, t31208: F, t31210: F, t31213: F, t31216: F, t31218: F, t31220: F, t31222: F, t31225: F) -> F {
    let t31230 = t18554 - F::cast_from(0.93011851851851851854e0_f64) * t18427 + t18555 - F::cast_from(0.89690000000000000001e0_f64) * t27262 + F::cast_from(0.82156666666666666665e0_f64) * t27292 + F::cast_from(0.11958666666666666667e1_f64) * t27295 - F::cast_from(0.3560484375e1_f64) * t31204 + F::cast_from(0.427258125e1_f64) * t31206 - F::cast_from(0.28483875e1_f64) * t31208 - F::cast_from(0.28483875e1_f64) * t31210 - F::cast_from(0.9494625e0_f64) * t31213 + F::cast_from(0.1151859375e0_f64) * t31216 - F::cast_from(0.230371875e0_f64) * t31218 + F::cast_from(0.46074375e0_f64) * t31220 + F::cast_from(0.46074375e0_f64) * t31222 + F::cast_from(0.15358125e0_f64) * t31225 - F::cast_from(0.29896666666666666667e0_f64) * t31067 + F::cast_from(0.8969e0_f64) * t31088 - F::cast_from(0.7302814814814814815e0_f64) * t18445;
    t31230
}
