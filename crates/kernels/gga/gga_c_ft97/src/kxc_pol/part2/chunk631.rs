//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 631/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk631<F: Float>(t1711: F, t371: F, t407: F, t391: F, t625: F, t68: F, t72: F, t2247: F, t47: F, t1675: F, t172: F, t173: F, t1743: F) -> (F, F, F, F, F, F, F, F) {
    let t8042 = t371 * t1711;
    let t8050 = t407 * t407;
    let t8051 = F::new(1.0) / t8050;
    let t8074 = t68 * t391 * t625 * t72;
    let t8076 = t47 * t2247;
    let t8078 = t68 * t8076 * t72;
    let t8079 = F::new(0.70937342644032921812e-2) * t8078;
    let t8086 = t68 * t1675 * t172 * t72;
    let t8098 = t173 * t1743;
    (t8042, t8051, t8074, t8076, t8078, t8079, t8086, t8098)
}
