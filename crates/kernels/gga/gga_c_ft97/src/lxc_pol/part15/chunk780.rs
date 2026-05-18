//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 780/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk780<F: Float>(t21416: F, t9707: F, t27: F, t89: F, t3717: F, t5053: F, t193: F, t18145: F, t18148: F, t18168: F, t18171: F, t18174: F, t21402: F, t21406: F, t21410: F, t21414: F) -> (F, F, F, F, F) {
    let t21417 = t9707 * t21416;
    let t21419 = t89 * t27 * t21417;
    let t21420 = t3717 * t5053;
    let t21422 = t89 * t193 * t21420;
    let t21428 = -t21402 / F::new(6.0) - t21406 / F::new(3.0) - t21410 / F::new(3.0) - t21414 / F::new(18.0) - t21419 + t21422 + t18148 / F::new(6.0) - t18145 / F::new(3.0) + t18168 / F::new(18.0) - t18171 / F::new(9.0) + t18174 / F::new(27.0);
    (t21417, t21419, t21420, t21422, t21428)
}
