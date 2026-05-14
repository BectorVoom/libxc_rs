//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 894/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk894<F: Float>(t20213: F, t2992: F, t11468: F, t11906: F, t11987: F, t11988: F, t12020: F, t12045: F, t16030: F, t16052: F, t1901: F, t1902: F, t1909: F, t20204: F, t20209: F, t20239: F, t39026: F, t4431: F, t4458: F, t4551: F, t47222: F, t75136: F, t75138: F, t75188: F, t8518: F, t85401: F, t85740: F, t925: F) -> (F, F) {
    let t85783 = t2992 * t20213;
    let t85789 = 8.0 / 9.0 * t1901 * t47222 * t20209 + 8.0 / 9.0 * t1901 * t16030 * t20204 - 16.0 / 9.0 * t1901 * t8518 * t12020 * t85740 + 8.0 / 3.0 * t1901 * t39026 * t75188 * t925 - 8.0 / 3.0 * t1901 * t11906 * t20239 - 4.0 / 3.0 * t1901 * t1909 * t12045 * t4431 * t4551 - 4.0 / 3.0 * t1901 * t1902 * t16052 * t4458 - 20.0 / 27.0 * t1901 * t11987 * t11988 * t85401 - 8.0 / 3.0 * t1901 * t11468 * t85783 + 4.0 / 27.0 * t75136 + 8.0 / 27.0 * t75138;
    (t85783, t85789)
}
