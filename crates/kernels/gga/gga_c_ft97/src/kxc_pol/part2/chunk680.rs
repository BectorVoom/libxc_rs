//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 680/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk680<F: Float>(t103: F, t7763: F, t11437: F, t11987: F, t1651: F, t3199: F, t1902: F, t11913: F, t11962: F, t11966: F, t11969: F, t11973: F, t11977: F, t11981: F, t11984: F, t1901: F, t446: F, t8499: F, t8516: F, t8523: F, t8526: F, t8534: F) -> (F,) {
    let t11988 = t103 * t7763;
    let t11989 = t11988 * t11437;
    let t11990 = t11987 * t11989;
    let t11993 = t3199 * t1651;
    let t11994 = t1902 * t11993;
    let t11997 = -t11913 - 2.0 / 27.0 * t8499 + 8.0 / 27.0 * t8516 + t8523 / 9.0 + t8526 / 27.0 - t8534 - t446 * t11962 / 3.0 + 2.0 / 3.0 * t446 * t11966 + 2.0 / 3.0 * t446 * t11969 - t446 * t11973 / 9.0 - 2.0 / 27.0 * t446 * t11977 - t11981 - 2.0 / 27.0 * t1901 * t11984 - 10.0 / 81.0 * t1901 * t11990 + t1901 * t11994 / 9.0;
    (t11997,)
}
