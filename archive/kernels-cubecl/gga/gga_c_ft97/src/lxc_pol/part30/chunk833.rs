//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 833/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk833<F: Float>(t35409: F, t420: F, t1127: F, t230: F, t7470: F, t27729: F, t6: F, t3766: F, t33444: F, t1113: F, t683: F, t224: F, t2427: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35410 = t420 * t35409;
    let t35414 = t230 * t1127;
    let t35415 = t420 * t35414;
    let t35416 = t7470 * t35415;
    let t35419 = t27729 * t6;
    let t35420 = t3766 * t35419;
    let t35426 = t3766 * t33444;
    let t35427 = t683 * t1113;
    let t35431 = t683 * t1127;
    let t35435 = t224 * t2427;
    (t35410, t35414, t35415, t35416, t35420, t35426, t35427, t35431, t35435)
}
