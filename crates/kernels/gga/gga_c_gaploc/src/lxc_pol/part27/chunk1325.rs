//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1325/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1325<F: Float>(t34582: F, t34363: F, t587: F, t912: F, t10615: F, t31158: F, t32033: F, t6963: F, t6964: F, t10526: F, t20471: F, t6540: F, t986: F) -> (F, F, F, F, F, F) {
    let t34583 = F::cast_from(0.38342925953920749676e0_f64) * t34582;
    let t34585 = t587 * t912 * t34363;
    let t34586 = F::cast_from(0.19171462976960374838e0_f64) * t34585;
    let t34587 = t10615 * t31158;
    let t34588 = F::cast_from(0.17875244975925213335e0_f64) * t34587;
    let t34592 = F::cast_from(0.85801175884441024006e1_f64) * t6963 * t6964 * t32033;
    let t34595 = F::cast_from(0.42900587942220512002e1_f64) * t20471 * t10526 * t32033;
    let t34600 = t6540 * t986;
    (t34583, t34586, t34588, t34592, t34595, t34600)
}
