//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 802/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk802<F: Float>(t16417: F, t16461: F, t457: F, t91: F, t1766: F, t4533: F, t473: F, t3119: F, t3157: F, t4505: F, t8345: F, t11043: F, t11076: F, t11404: F, t11946: F, t11957: F, t8260: F, t8451: F) -> (F, F, F, F, F) {
    let t16462 = t16417 + t16461;
    let t16464 = t91 * t457 * t16462;
    let t16467 = t1766 * t4533;
    let t16469 = t91 * t16467 * t473;
    let t16472 = t91 * t3119 * t3157;
    let t16474 = t8345 * t4505;
    let t16476 = t91 * t16474 * t473;
    let t16478 = -t8451 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11043 + t11946 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t11076 - t8260 + t16464 / F::cast_from(2.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11404 - t11957 - t16469 / F::cast_from(4.0_f64) - t16472 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t16476;
    (t16464, t16469, t16472, t16476, t16478)
}
