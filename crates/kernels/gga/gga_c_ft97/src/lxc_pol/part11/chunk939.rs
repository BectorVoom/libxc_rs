//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 939/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk939<F: Float>(t41950: F, t2366: F, t89: F, t9733: F, t1636: F, t2344: F, t2350: F, t9725: F, t9750: F, t375: F, t9567: F, t9718: F, t241: F, t41446: F, t41448: F, t9716: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t41951 = 140.0 / 243.0 * t41950;
    let t41953 = t89 * t9733 * t2366;
    let t41954 = 4.0 / 27.0 * t41953;
    let t41955 = t1636 * t2344;
    let t41957 = t89 * t41955 * t2350;
    let t41958 = 8.0 / 81.0 * t41957;
    let t41960 = t89 * t9725 * t9750;
    let t41962 = t375 * t9567;
    let t41964 = t89 * t41962 * t9718;
    let t41966 = t241 * t41446;
    let t41969 = t89 * t9716 * t41966 * t41448;
    (t41951, t41953, t41954, t41955, t41957, t41958, t41960, t41962, t41964, t41969)
}
