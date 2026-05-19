//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1055/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1055<F: Float>(t1181: F, t2068: F, t23445: F, t604: F, t30613: F, t30468: F, t4425: F, t4685: F, t7822: F, t4331: F, t1470: F, t30644: F) -> (F, F, F, F, F, F) {
    let t34497 = t2068 * t1181 * t604 * t23445;
    let t34499 = F::cast_from(0.25724410870841842184e-2_f64) * t30613;
    let t34500 = t30468 * t4425;
    let t34501 = F::cast_from(0.34299214494455789578e-2_f64) * t34500;
    let t34502 = t7822 * t4685;
    let t34504 = t7822 * t4331;
    let t34506 = t30644 * t1470;
    (t34497, t34499, t34501, t34502, t34504, t34506)
}
