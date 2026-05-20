//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3270/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3270<F: Float>(t4423: F, t775: F, t40791: F, t5989: F, t10890: F, t5985: F, t124: F, t14586: F, t14791: F, t221: F, t2730: F, t36833: F, t40782: F, t40784: F, t40792: F, t4343: F, t4362: F, t4433: F, t50446: F, t50977: F, t50982: F, t51049: F, t61234: F, t800: F) -> (F, F) {
    let t62080 = t4423 * t775;
    let t62089 = t40791 * t5989;
    let t62095 = t10890 * t5985;
    let t62101 = F::cast_from(0.4065600224742826258e-4_f64) * t50977 + F::new(7.0) / F::new(72.0) * t50982 + F::cast_from(0.30234122406223992295e0_f64) * t40782 + F::cast_from(0.1133779590233399711e0_f64) * t40784 - F::cast_from(0.68598428988911579156e-2_f64) * t4362 * t14791 * t14586 * t62080 + F::cast_from(0.51448821741683684367e-2_f64) * t4362 * t36833 * t14586 * t51049 + F::new(35.0) / F::new(72.0) * t62089 + t2730 * t800 * t124 * t61234 / F::new(8.0) - F::new(35.0) / F::new(216.0) * t62095 + F::new(35.0) / F::new(72.0) * t40792 - t50446 * t221 * t4433 * t4343;
    (t62080, t62101)
}
