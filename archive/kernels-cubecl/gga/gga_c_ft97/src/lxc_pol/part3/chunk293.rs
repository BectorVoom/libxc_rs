//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 293/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk293<F: Float>(t1168: F, t762: F, t242: F, t1140: F, t1144: F, t1162: F, t193: F, t446: F, t723: F, t89: F, t1160: F, t258: F) -> (F, F, F, F) {
    let t1169 = t762 * t1168;
    let t1170 = t242 * t1169;
    let t1173 = -t723 - t446 * t1140 / F::cast_from(9.0_f64) - t446 * t1144 / F::cast_from(3.0_f64) + t89 * t193 * t1162 / F::cast_from(3.0_f64) - t446 * t1170 / F::cast_from(3.0_f64);
    let t1175 = t1160 * t258;
    (t1169, t1170, t1173, t1175)
}
