//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 917/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk917<F: Float>(t10007: F, t14192: F, t2492: F, t265: F, t13702: F, t9802: F, t13706: F, t2409: F, t3869: F, t2606: F, t10000: F, t10012: F, t14156: F, t14160: F, t14164: F, t14168: F, t14172: F, t14177: F, t14184: F, t14189: F, t1901: F, t9997: F) -> F {
    let t14193 = t10007 * t14192;
    let t14196 = t2492 * t265;
    let t14197 = t14196 * t13702;
    let t14200 = t9802 * t265;
    let t14201 = t14200 * t13706;
    let t14205 = t3869 * t2409;
    let t14206 = t2606 * t14205;
    let t14209 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t14156 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t14160 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t14164 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t14168 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t14172 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t14177 - t9997 / F::cast_from(9.0_f64) + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t10000 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t14184 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t14189 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t14193 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t14197 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t14201 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10012 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t14206;
    t14209
}
