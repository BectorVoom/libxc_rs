//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 938/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk938<F: Float>(t18502: F, t2599: F, t4917: F, t766: F, t9791: F, t2606: F, t11593: F, t14114: F, t18455: F, t18457: F, t18461: F, t18464: F, t18468: F, t18473: F, t18476: F, t18479: F, t18483: F, t18488: F, t18493: F, t18499: F, t1901: F, t446: F) -> (F, F) {
    let t18503 = t2599 * t18502;
    let t18506 = t4917 * t766;
    let t18507 = t9791 * t18506;
    let t18508 = t2606 * t18507;
    let t18511 = -F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t18455 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t18457 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t18461 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t18464 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t18468 + t1901 * t18473 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t18476 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t18479 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t18483 + t446 * t18488 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18493 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t14114 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t11593 * t18499 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t18503 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t18508;
    (t18506, t18511)
}
