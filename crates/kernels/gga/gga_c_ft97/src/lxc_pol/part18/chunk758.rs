//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 758/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk758<F: Float>(t18: F, t609: F, t2211: F, t2210: F, t11593: F, t12941: F, t12947: F, t12952: F, t12958: F, t12963: F, t12965: F, t12967: F, t12971: F, t12975: F, t12976: F, t12979: F, t12983: F, t12988: F, t1901: F, t28: F, t446: F, t89: F, t9112: F) -> (F, F, F) {
    let t12991 = t18 * t609;
    let t12992 = t2211 * t12991;
    let t12993 = t2210 * t12992;
    let t12996 = t89 * t28 * t12941 / 3.0 + 2.0 / 3.0 * t446 * t12947 + t446 * t12952 / 3.0 - 2.0 / 27.0 * t9112 + 2.0 / 3.0 * t446 * t12958 - t12963 - t12965 - t12967 - 4.0 / 3.0 * t1901 * t12971 - t12975 + 2.0 / 9.0 * t1901 * t12976 + 4.0 / 9.0 * t1901 * t12979 - 4.0 / 27.0 * t1901 * t12983 + 4.0 / 9.0 * t11593 * t12988 + 4.0 / 9.0 * t11593 * t12993;
    (t12991, t12992, t12996)
}
