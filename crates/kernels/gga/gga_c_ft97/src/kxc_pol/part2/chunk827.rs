//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 827/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk827<F: Float>(t18: F, t558: F, t2222: F, t2221: F, t609: F, t2211: F, t2210: F, t11593: F, t12941: F, t12947: F, t12952: F, t12958: F, t12963: F, t12965: F, t12967: F, t12971: F, t12975: F, t12976: F, t12979: F, t12983: F, t1901: F, t28: F, t446: F, t89: F, t9112: F) -> F {
    let t12986 = t18 * t558;
    let t12987 = t2222 * t12986;
    let t12988 = t2221 * t12987;
    let t12991 = t18 * t609;
    let t12992 = t2211 * t12991;
    let t12993 = t2210 * t12992;
    let t12996 = t89 * t28 * t12941 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t12947 + t446 * t12952 / F::new(3.0) - F::new(2.0) / F::new(27.0) * t9112 + F::new(2.0) / F::new(3.0) * t446 * t12958 - t12963 - t12965 - t12967 - F::new(4.0) / F::new(3.0) * t1901 * t12971 - t12975 + F::new(2.0) / F::new(9.0) * t1901 * t12976 + F::new(4.0) / F::new(9.0) * t1901 * t12979 - F::new(4.0) / F::new(27.0) * t1901 * t12983 + F::new(4.0) / F::new(9.0) * t11593 * t12988 + F::new(4.0) / F::new(9.0) * t11593 * t12993;
    t12996
}
