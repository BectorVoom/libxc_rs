//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 600/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk600<F: Float>(t1101: F, t215: F, t206: F, t214: F, t52: F, t204: F, t41: F, t4995: F, t237: F, t1100: F, t213: F, t11: F) -> (F, F, F, F, F, F) {
    let t4999 = t215 * t1101;
    let t5001 = F::new(1.0) / t206 / t4999;
    let t5003 = t52 * t214 * t5001;
    let t5005 = F::cast_from(0.44057546758024691357e0_f64) * t41 * t204 * t4995 + F::cast_from(0.18770038718167957794e-1_f64) * t5003;
    let t5006 = t237 * t5005;
    let t5007 = t1100 * t5006;
    let t5009 = t213 * t213;
    let t5010 = t11 * t5009;
    (t5001, t5003, t5005, t5007, t5009, t5010)
}
