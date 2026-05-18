//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 708/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk708<F: Float>(t12310: F, t12327: F, t12356: F, t12365: F, t157: F, t526: F, t3421: F, t8392: F, t1045: F, t2101: F, t1055: F, t8232: F) -> (F, F, F, F, F, F, F, F) {
    let t13102 = F::new(4.0) / F::new(27.0) * t12310;
    let t13108 = F::new(2.0) / F::new(9.0) * t12327;
    let t13117 = F::new(4.0) / F::new(3.0) * t12356;
    let t13120 = F::new(2.0) / F::new(3.0) * t12365;
    let t13140 = t526 * t157;
    let t13152 = F::new(2.0) / F::new(27.0) * t8392 * t3421;
    let t13153 = t2101 * t1045;
    let t13187 = t8232 * t1055;
    (t13102, t13108, t13117, t13120, t13140, t13152, t13153, t13187)
}
