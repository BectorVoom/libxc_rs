//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 810/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk810<F: Float>(t1882: F, t7626: F, t7622: F, t1901: F, t34156: F, t34158: F, t34160: F, t34164: F, t34169: F, t34174: F, t34178: F, t34183: F, t34187: F, t34191: F, t446: F) -> (F, F, F) {
    let t34193 = F::new(2.0) / F::new(9.0) * t1882 * t7626;
    let t34195 = F::new(2.0) / F::new(9.0) * t1882 * t7622;
    let t34196 = t34156 - t34158 - F::new(2.0) / F::new(9.0) * t1901 * t34160 + F::new(2.0) / F::new(3.0) * t446 * t34164 - F::new(2.0) / F::new(3.0) * t446 * t34169 - F::new(2.0) * t446 * t34174 - F::new(2.0) * t446 * t34178 - F::new(2.0) / F::new(3.0) * t446 * t34183 + F::new(4.0) / F::new(3.0) * t446 * t34187 + t34191 + t34193 - t34195;
    (t34193, t34195, t34196)
}
