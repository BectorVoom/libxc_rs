//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1206/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1206<F: Float>(t5393: F, t2843: F, t1091: F, t1255: F, t1901: F, t21362: F, t22194: F, t2857: F, t296: F, t319: F, t44528: F, t446: F, t5299: F, t5309: F, t72805: F, t835: F, t840: F, t84940: F, t84958: F, t84983: F, t84985: F, t88735: F, t88749: F) -> (F, F) {
    let t91124 = t5393 * t5393;
    let t91125 = t2843 * t91124;
    let t91136 = -F::cast_from(4.0_f64) * t446 * t840 * t2843 * t5299 * t5309 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t835 * t1255 * t21362 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t2857 * t319 * t88735 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t84940 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t44528 * t22194 * t1091 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t84958 + F::cast_from(2.0_f64) * t446 * t296 * t91125 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t72805 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t835 * t319 * t88749 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t84983 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t84985;
    (t91125, t91136)
}
