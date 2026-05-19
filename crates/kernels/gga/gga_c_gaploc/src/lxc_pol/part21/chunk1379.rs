//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1379/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1379<F: Float>(t12063: F, t1359: F, t1424: F, t34143: F, t34145: F, t34148: F, t34151: F, t34153: F, t34156: F, t34178: F, t34181: F, t34186: F, t34189: F, t34191: F, t34216: F, t34220: F, t34242: F, t34245: F, t544: F) -> F {
    let t38481 = -t34143 - t34145 - t34148 - t34151 - t34153 - t34156 + t34178 - t34181 - t34186 - t34189 - t34191 - t34216 - t34220 - F::cast_from(0.79445533226334281486e-1_f64) * t544 * t1359 * t12063 * t1424 - t34242 + t34245;
    t38481
}
