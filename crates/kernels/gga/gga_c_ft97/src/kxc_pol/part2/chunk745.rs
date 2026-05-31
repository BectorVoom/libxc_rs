//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 745/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk745<F: Float>(t11054: F, t8291: F, t10979: F, t1787: F, t10983: F, t10988: F, t8327: F, t11665: F, t11668: F, t11669: F, t11672: F, t11676: F, t11684: F, t11686: F, t11687: F, t11691: F, t11694: F, t11697: F, t11700: F, t3051: F, t3139: F, t462: F, t8283: F, t8285: F, t8287: F, t8333: F, t92: F) -> F {
    let t11703 = t8291 * t11054;
    let t11706 = t1787 * t10979;
    let t11709 = t1787 * t10983;
    let t11712 = t8327 * t10988;
    let t11715 = -t92 * t11665 + t11668 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11669 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3051 * t11672 + F::cast_from(2.0_f64) * t462 * t11676 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8283 + t8285 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8287 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t8333 - t11684 + t11686 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t462 * t11687 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t462 * t11691 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3139 * t11694 + t462 * t11697 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t11700 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t11703 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t11706 + t462 * t11709 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t462 * t11712;
    t11715
}
