//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 788/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk788<F: Float>(t241: F, t41536: F, t10: F, t11175: F, t242: F, t1636: F, t2344: F, t375: F, t9567: F, t41446: F, t190: F, t2371: F, t251: F, t36452: F, t37991: F, t2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41912 = t241 * t41536;
    let t41950 = t10 * t11175 * t242;
    let t41951 = 140.0 / 243.0 * t41950;
    let t41955 = t1636 * t2344;
    let t41962 = t375 * t9567;
    let t41966 = t241 * t41446;
    let t42044 = 280.0 / 243.0 * t41950;
    let t42050 = 1.0 / t251 / t37991 / t190 / t2371 / t36452 / 96.0;
    let t42087 = t2 * t41446;
    (t41912, t41950, t41951, t41955, t41962, t41966, t42044, t42050, t42087)
}
