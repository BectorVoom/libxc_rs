//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1020/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1020<F: Float>(t1781: F, t85451: F, t1791: F, t8276: F, t85469: F, t1780: F, t38525: F, t462: F, t463: F, t73497: F, t73504: F, t73506: F, t73508: F, t73574: F, t73576: F, t8275: F, t86023: F, t86027: F, t86031: F) -> (F, F, F, F) {
    let t86035 = t1781 * t85451;
    let t86039 = t1791 * t85451;
    let t86043 = t8276 * t85469;
    let t86052 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t73497 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t462 * t8275 * t86023 + F::cast_from(8.0_f64) * t462 * t463 * t86027 - t462 * t463 * t86031 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t1780 * t86035 + F::cast_from(2.0_f64) * t462 * t463 * t86039 - F::cast_from(8.0_f64) * t462 * t1780 * t86043 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t73504 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t73506 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t73508 + t38525 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t73574 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t73576;
    (t86035, t86039, t86043, t86052)
}
