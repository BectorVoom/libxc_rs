//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1297/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1297<F: Float>(t22230: F, t22290: F, t22800: F, t22811: F, t22812: F, t27358: F, t27361: F, t27370: F, t27373: F, t31240: F, t31242: F, t31250: F, t31254: F, t31258: F, t31262: F, t31265: F, t31268: F, t31271: F, t31273: F) -> F {
    let t31575 = -F::new(0.48204333333333333333e1) * t22230 + t22800 - F::new(0.27785333333333333333e1) * t22290 + t22811 + t22812 + F::new(0.3529725e1) * t31240 + F::new(0.6311625e0) * t31242 + F::new(0.104195e1) * t27358 - F::new(0.125034e1) * t27361 - F::new(0.62517e0) * t27370 - F::new(0.62517e0) * t27373 + F::new(0.937755e0) * t31250 + F::new(0.937755e0) * t31254 + F::new(0.312585e0) * t31258 + F::new(0.312585e0) * t31262 - F::new(0.62517e0) * t31265 - F::new(0.20839e0) * t31268 + F::new(0.794188125e1) * t31271 - F::new(0.473371875e0) * t31273;
    t31575
}
