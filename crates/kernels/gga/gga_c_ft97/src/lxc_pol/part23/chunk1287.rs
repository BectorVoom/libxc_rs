//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1287/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1287<F: Float>(t31063: F, t41402: F, t2568: F, t31060: F, t766: F, t1882: F, t31203: F, t111215: F, t111221: F, t111223: F, t111225: F, t111227: F, t111237: F, t124098: F, t1456: F, t18459: F, t18491: F, t18506: F, t18622: F, t1901: F, t242: F, t2574: F, t265: F, t28299: F, t28300: F, t28344: F, t42362: F, t446: F, t4969: F, t51687: F, t6194: F, t724: F) -> (F, F, F) {
    let t124895 = t41402 * t31063;
    let t124900 = t2568 * t31060 * t766;
    let t124908 = t1882 * t31203;
    let t124914 = -2.0 / 27.0 * t1901 * t42362 * t28344 * t18459 - 4.0 / 27.0 * t1901 * t51687 * t28344 * t18506 + 4.0 / 3.0 * t446 * t2574 * t1456 * t18622 - 8.0 / 27.0 * t111215 + t111221 + 4.0 / 3.0 * t446 * t2574 * t265 * t124098 - 2.0 * t446 * t242 * t124895 + t111223 - t111225 + t111227 + 2.0 / 3.0 * t446 * t242 * t124900 + 2.0 / 9.0 * t446 * t724 * t6194 * t4969 + t124908 / 27.0 - 2.0 * t1901 * t28299 * t28300 * t18491 - t111237;
    (t124895, t124900, t124914)
}
