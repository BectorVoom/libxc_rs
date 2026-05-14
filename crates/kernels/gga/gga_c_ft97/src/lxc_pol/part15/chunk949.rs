//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 949/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk949<F: Float>(t1053: F, t167: F, t1901: F, t20655: F, t20709: F, t20723: F, t20727: F, t3578: F, t40792: F, t41002: F, t446: F, t4668: F, t4714: F, t4823: F, t569: F, t574: F, t605: F, t63586: F, t76623: F, t85516: F, t85546: F, t85554: F, t9144: F, t925: F, t9327: F, t9432: F) -> (F,) {
    let t87441 = 2.0 / 3.0 * t446 * t569 * t167 * t85546 - 80.0 / 243.0 * t446 * t41002 * t167 * t85554 + 8.0 / 9.0 * t76623 - 12.0 * t446 * t9432 * t167 * t4668 * t4714 + 4.0 * t446 * t574 * t3578 * t20723 + 4.0 * t446 * t574 * t3578 * t20727 + 4.0 / 3.0 * t446 * t574 * t605 * t20655 * t1053 + 40.0 / 27.0 * t446 * t9327 * t167 * t85516 + 8.0 / 3.0 * t1901 * t40792 * t20709 * t925 - 4.0 / 3.0 * t1901 * t9144 * t20727 * t925 + 4.0 / 3.0 * t1901 * t63586 * t4823;
    (t87441,)
}
