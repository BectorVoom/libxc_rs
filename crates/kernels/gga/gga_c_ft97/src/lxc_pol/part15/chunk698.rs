//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 698/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk698<F: Float>(t1852: F, t20268: F, t83: F, t11550: F, t11578: F, t16192: F, t16213: F, t1901: F, t20226: F, t20230: F, t20233: F, t20236: F, t20240: F, t20244: F, t20248: F, t20256: F, t20260: F, t20265: F, t446: F) -> (F, F, F) {
    let t20269 = t1852 * t20268;
    let t20270 = t83 * t20269;
    let t20273 = F::new(2.0) / F::new(3.0) * t1901 * t20226 - F::new(2.0) / F::new(9.0) * t1901 * t20230 + F::new(2.0) / F::new(3.0) * t1901 * t20233 + F::new(2.0) / F::new(3.0) * t1901 * t20236 - F::new(2.0) / F::new(3.0) * t1901 * t20240 + F::new(4.0) / F::new(9.0) * t446 * t20244 + F::new(2.0) / F::new(3.0) * t446 * t20248 - F::new(4.0) / F::new(9.0) * t11550 - F::new(2.0) / F::new(9.0) * t16192 + F::new(4.0) / F::new(9.0) * t11578 + t16213 / F::new(3.0) - t446 * t20256 / F::new(9.0) - F::new(10.0) / F::new(81.0) * t446 * t20260 - F::new(2.0) * t446 * t20265 + F::new(2.0) * t446 * t20270;
    (t20269, t20270, t20273)
}
