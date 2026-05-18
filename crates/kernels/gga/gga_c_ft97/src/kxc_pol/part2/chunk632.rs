//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 632/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk632<F: Float>(t419: F, t8098: F, t424: F, t626: F, t173: F, t1747: F, t1738: F, t1570: F, t23: F, t1675: F, t67: F, t9: F) -> (F, F, F, F, F, F) {
    let t8099 = t419 * t8098;
    let t8109 = t626 * t424;
    let t8110 = t419 * t8109;
    let t8112 = t173 * t1747;
    let t8113 = t419 * t8112;
    let t8115 = t173 * t1738;
    let t8116 = t419 * t8115;
    let t8119 = F::new(1.0) / t23 / t1570;
    let t8130 = t9 * t67 * t1675;
    (t8099, t8110, t8113, t8116, t8119, t8130)
}
