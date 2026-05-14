//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 599/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk599<F: Float>(t110: F, t1871: F, t26001: F, t5635: F, t986: F, t26048: F, t83: F, t6535: F, t8392: F, t487: F, t6557: F, t379: F, t1909: F, t23327: F, t3200: F, t1901: F, t26230: F, t26234: F, t26237: F, t26242: F, t26246: F, t26249: F, t26252: F, t446: F) -> (F, F) {
    let t26255 = t1871 * t110 * t26001;
    let t26259 = t1871 * t986 * t5635;
    let t26262 = t83 * t26048;
    let t26265 = t8392 * t6535;
    let t26267 = t487 * t6557;
    let t26268 = t26267 * t379;
    let t26269 = t1909 * t26268;
    let t26272 = t23327 * t3200;
    let t26275 = 2.0 / 3.0 * t446 * t26230 + t446 * t26234 / 3.0 + 2.0 / 3.0 * t446 * t26237 + t446 * t26242 / 3.0 + t1901 * t26246 / 9.0 + t1901 * t26249 / 9.0 - 2.0 / 9.0 * t26252 + 2.0 / 3.0 * t446 * t26255 + 2.0 / 3.0 * t446 * t26259 + 2.0 / 3.0 * t446 * t26262 - t26265 / 27.0 + t1901 * t26269 / 9.0 + t1901 * t26272 / 9.0;
    (t26268, t26275)
}
