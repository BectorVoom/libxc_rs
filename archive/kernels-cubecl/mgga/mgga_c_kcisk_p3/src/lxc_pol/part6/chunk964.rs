//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 964/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk964<F: Float>(t29494: F, t29496: F, t29499: F, t29501: F, t29505: F, t29507: F, t29514: F, t29517: F, t29520: F, t29524: F, t29526: F, t29529: F, t29531: F, t29535: F, t29537: F) -> F {
    let t30082 = F::cast_from(0.9375e-1_f64) * t29494 + F::cast_from(0.43166666666666666667e0_f64) * t29496 - F::cast_from(0.50000000000000000001e0_f64) * t29499 - F::cast_from(0.375e0_f64) * t29501 - F::cast_from(0.9375e-1_f64) * t29505 + F::cast_from(0.275e1_f64) * t29507 + F::cast_from(0.25060648148148148148e1_f64) * t29514 + F::cast_from(0.375e0_f64) * t29517 + F::cast_from(0.71944444444444444444e-1_f64) * t29520 + F::cast_from(0.29976851851851851851e-2_f64) * t29524 - F::cast_from(0.625e-1_f64) * t29526 - F::cast_from(0.275e1_f64) * t29529 - F::cast_from(0.60703125e-1_f64) * t29531 + F::cast_from(0.101171875e-1_f64) * t29535 + F::cast_from(0.303515625e-1_f64) * t29537;
    t30082
}
