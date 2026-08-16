//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 799/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk799<F: Float>(t14224: F, t18593: F, t1901: F, t21641: F, t21647: F, t21652: F, t21657: F, t21661: F, t21665: F, t21669: F, t21674: F, t21678: F, t21682: F, t21686: F, t21689: F, t21693: F, t446: F) -> F {
    let t21696 = -t446 * t21641 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18593 + t1901 * t21647 / F::cast_from(3.0_f64) + t1901 * t21652 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t21657 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t21661 + t1901 * t21665 / F::cast_from(3.0_f64) + t1901 * t21669 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t21674 + F::cast_from(2.0_f64) * t446 * t21678 - t446 * t21682 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t14224 - t446 * t21686 + F::cast_from(2.0_f64) * t446 * t21689 - t446 * t21693 / F::cast_from(3.0_f64);
    t21696
}
