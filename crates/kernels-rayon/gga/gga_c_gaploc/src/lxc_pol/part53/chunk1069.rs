//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1069/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1069(t224: f64, t51061: f64, t51063: f64, t51072: f64, t51198: f64, t41574: f64, t41575: f64, t41579: f64, t41581: f64, t41585: f64, t41586: f64, t42470: f64, t42473: f64, t42475: f64, t42481: f64, t42483: f64, t42485: f64, t42487: f64, t42491: f64, t42494: f64, t42496: f64, t50808: f64, t50809: f64, t50811: f64) -> (f64, f64) {
    let t51201 = t224 * (t51061 + t51063 + t51072 + t51198);
    let t51232 = -t41574 - t41575 + t50808 - t41579 + t41581 - t50809 - t41585 - t41586 - t42470 - t50811 - t42473 + t42475 - t42481 + t42483 - t42485 + t42487 + t42491 + t42494 + t42496;
    (t51201, t51232)
}
