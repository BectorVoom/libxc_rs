//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3590/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3590<F: Float>(t68287: F, t68292: F, t68297: F, t68301: F, t68305: F, t68310: F, t68312: F, t68315: F, t68319: F, t68322: F, t68326: F, t68330: F, t68332: F, t68334: F, t68336: F) -> F {
    let t68338 = -F::cast_from(0.20128333333333333333e0_f64) * t68287 - F::new(0.12077e1) * t68292 + F::new(0.12077e1) * t68297 + F::new(0.60385e0) * t68301 + F::new(0.181155e1) * t68305 - F::cast_from(0.89459259259259259259e0_f64) * t68310 + F::cast_from(0.36793333333333333334e-1_f64) * t68312 + F::new(0.33114e0) * t68315 + F::new(0.49671e0) * t68319 + F::new(0.198684e1) * t68322 - F::new(0.16557e0) * t68326 - F::new(0.99342e0) * t68330 + F::cast_from(0.13418888888888888889e0_f64) * t68332 + F::cast_from(0.26837777777777777778e0_f64) * t68334 + F::cast_from(0.80513333333333333333e0_f64) * t68336;
    t68338
}
