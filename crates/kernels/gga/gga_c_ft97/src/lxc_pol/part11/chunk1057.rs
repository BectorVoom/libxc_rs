//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1057/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1057<F: Float>(t2354: F, t41945: F, t446: F, t10: F, t11175: F, t242: F, t2366: F, t89: F, t9733: F, t1636: F, t2344: F, t2350: F) -> (F, F, F, F, F, F, F) {
    let t41947 = t446 * t2354 * t41945;
    let t41950 = t10 * t11175 * t242;
    let t41951 = F::cast_from(140.0_f64) / F::cast_from(243.0_f64) * t41950;
    let t41953 = t89 * t9733 * t2366;
    let t41954 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t41953;
    let t41955 = t1636 * t2344;
    let t41957 = t89 * t41955 * t2350;
    (t41947, t41950, t41951, t41953, t41954, t41955, t41957)
}
