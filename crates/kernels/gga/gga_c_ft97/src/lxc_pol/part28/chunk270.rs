//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 270/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk270<F: Float>(t2993: F, t447: F, t446: F, t14: F, t1576: F, t17: F, t355: F, t18: F, t359: F, t89: F, t375: F, t943: F) -> (F, F, F, F, F, F) {
    let t2994 = t447 * t2993;
    let t2995 = t446 * t2994;
    let t2998 = F::cast_from(1.0_f64) / t14 / t1576;
    let t2999 = t2998 * t17;
    let t3000 = t2999 * t355;
    let t3001 = t359 * t18;
    let t3003 = t89 * t3000 * t3001;
    let t3006 = t89 * t375 * t943;
    (t2995, t2998, t2999, t3000, t3003, t3006)
}
