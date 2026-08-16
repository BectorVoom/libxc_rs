//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 956/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk956<F: Float>(t1882: F, t33734: F, t33717: F, t8392: F, t33613: F, t33768: F, t7495: F, t8232: F, t2399: F, t7538: F, t89: F, t33660: F) -> (F, F, F, F, F, F, F) {
    let t142240 = t1882 * t33734;
    let t142267 = t8392 * t33717;
    let t142269 = t1882 * t33613;
    let t142296 = t1882 * t33768;
    let t142326 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8232 * t7495;
    let t142333 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t89 * t2399 * t7538;
    let t142334 = t1882 * t33660;
    (t142240, t142267, t142269, t142296, t142326, t142333, t142334)
}
