//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 920/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk920<F: Float>(t224: F, t51061: F, t51063: F, t51072: F, t51198: F, t41574: F, t41575: F, t41579: F, t41581: F, t41585: F, t41586: F, t42470: F, t42473: F, t42475: F, t42481: F, t42483: F, t42485: F, t42487: F, t42491: F, t42494: F, t42496: F, t50808: F, t50809: F, t50811: F) -> (F, F) {
    let t51201 = t224 * (t51061 + t51063 + t51072 + t51198);
    let t51232 = -t41574 - t41575 + t50808 - t41579 + t41581 - t50809 - t41585 - t41586 - t42470 - t50811 - t42473 + t42475 - t42481 + t42483 - t42485 + t42487 + t42491 + t42494 + t42496;
    (t51201, t51232)
}
