//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1284/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1284<F: Float>(t31270: F, t7996: F, t7999: F, t22230: F, t22290: F, t22293: F, t22296: F, t22336: F, t27358: F, t27361: F, t27370: F, t27373: F, t31240: F, t31242: F, t31250: F, t31254: F, t31258: F, t31262: F, t31265: F, t31268: F) -> (F, F, F) {
    let t31271 = t7996 * t31270;
    let t31273 = t7999 * t31270;
    let t31275 = -F::cast_from(0.27903555555555555556e1_f64) * t22230 + t22336 - F::cast_from(0.21908444444444444444e1_f64) * t22290 + F::cast_from(0.82156666666666666666e0_f64) * t22293 + F::cast_from(0.82156666666666666666e0_f64) * t22296 + F::new(0.1898925e1) * t31240 + F::new(0.3071625e0) * t31242 + F::cast_from(0.82156666666666666665e0_f64) * t27358 - F::cast_from(0.98587999999999999998e0_f64) * t27361 - F::cast_from(0.49293999999999999999e0_f64) * t27370 - F::cast_from(0.49293999999999999999e0_f64) * t27373 + F::new(0.73941e0) * t31250 + F::new(0.73941e0) * t31254 + F::new(0.24647e0) * t31258 + F::new(0.24647e0) * t31262 - F::new(0.49294e0) * t31265 - F::cast_from(0.16431333333333333333e0_f64) * t31268 + F::cast_from(0.427258125e1_f64) * t31271 - F::cast_from(0.230371875e0_f64) * t31273;
    (t31271, t31273, t31275)
}
