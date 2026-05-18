//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 692/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk692<F: Float>(t1011: F, t1015: F, t1022: F, t11: F, t12408: F, t12410: F, t12414: F, t12425: F, t12436: F, t12442: F, t12446: F, t12450: F, t139: F, t157: F, t197: F, t201: F, t3125: F, t3190: F, t3200: F, t3207: F, t3217: F, t972: F) -> F {
    let t12453 = -F::new(0.74295e-1) * t12408 * t12410 - F::new(0.4953e-1) * t3207 * t12414 - F::new(0.15918666666666666666e0) * t139 * t11 * t3125 - F::new(0.79593333333333333331e-1) * t139 * t201 * t12425 + F::new(0.5306222222222222222e-1) * t139 * t157 * t972 - F::new(0.1857375e-1) * t3190 * t1022 - F::new(0.371475e-1) * t197 * t12436 + F::new(0.371475e-1) * t1011 * t3217 - F::new(0.8255e-2) * t3200 * t12442 + F::new(0.371475e-1) * t3207 * t12446 - F::new(0.38523333333333333333e-1) * t1015 * t12450;
    t12453
}
