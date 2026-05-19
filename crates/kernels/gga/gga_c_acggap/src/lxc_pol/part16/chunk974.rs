//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 974/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk974<F: Float>(t34487: F, t7380: F, t1165: F, t33509: F, t604: F, t7346: F, t30468: F, t4425: F, t1470: F, t30644: F, t30984: F, t8458: F) -> (F, F, F, F, F) {
    let t34488 = t7380 * t34487;
    let t34489 = F::new(0.4584375e-1) * t34488;
    let t34492 = t7346 * t1165 * t604 * t33509;
    let t34500 = t30468 * t4425;
    let t34501 = F::cast_from(0.34299214494455789578e-2_f64) * t34500;
    let t34506 = t30644 * t1470;
    let t34507 = F::cast_from(0.17149607247227894789e-2_f64) * t34506;
    let t34508 = t30984 * t8458;
    (t34489, t34492, t34501, t34507, t34508)
}
