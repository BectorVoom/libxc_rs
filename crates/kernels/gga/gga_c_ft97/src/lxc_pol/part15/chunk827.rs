//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 827/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk827<F: Float>(t55558: F, t55562: F, t5419: F, t8232: F, t2842: F, t5374: F, t5395: F, t848: F, t38953: F, t5410: F, t5399: F, t2252: F, t342: F, t5202: F, t1526: F, t42262: F, t5198: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t72080 = 56.0 / 81.0 * t55558;
    let t72082 = 56.0 / 243.0 * t55562;
    let t72167 = t8232 * t5419;
    let t72231 = t5374 * t2842;
    let t72263 = t8232 * t5395;
    let t72391 = t848 * t5374;
    let t72523 = t38953 * t5410;
    let t72805 = t8232 * t5399;
    let t72977 = t342 * t2252 * t5202;
    let t72992 = t1526 * t42262 * t5198;
    (t72080, t72082, t72167, t72231, t72263, t72391, t72523, t72805, t72977, t72992)
}
