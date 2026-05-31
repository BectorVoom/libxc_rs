//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 643/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk643<F: Float>(t2235: F, t5: F, t177: F, t2280: F, t11: F, t2247: F, t41: F) -> (F, F, F) {
    let t8614 = t5 * t2235;
    let t8618 = F::cast_from(1.0_f64) / t2280 / t177;
    let t8639 = t11 * t2247;
    let t8640 = t41 * t8639;
    (t8614, t8618, t8640)
}
