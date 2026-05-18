//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 695/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk695<F: Float>(t10992: F, t11021: F, t11023: F, t11025: F, t11069: F, t11416: F, t3206: F, t8392: F, t100: F, t8275: F, t103: F, t7763: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11922 = F::new(2.0) / F::new(9.0) * t10992;
    let t11930 = F::new(2.0) / F::new(9.0) * t11021;
    let t11931 = F::new(4.0) / F::new(9.0) * t11023;
    let t11932 = F::new(4.0) / F::new(27.0) * t11025;
    let t11946 = F::new(2.0) / F::new(3.0) * t11069;
    let t11957 = F::new(4.0) / F::new(3.0) * t11416;
    let t11981 = F::new(2.0) / F::new(27.0) * t8392 * t3206;
    let t11987 = t8275 * t100;
    let t11988 = t103 * t7763;
    (t11922, t11930, t11931, t11932, t11946, t11957, t11981, t11987, t11988)
}
