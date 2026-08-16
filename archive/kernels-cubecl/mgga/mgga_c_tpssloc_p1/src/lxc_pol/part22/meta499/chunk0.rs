//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1931/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1931<F: Float>(t21251: F, t21255: F, t21263: F, t21265: F, t21267: F, t21270: F, t21302: F, t21305: F, t21317: F, t21320: F, t21336: F, t21372: F) -> F {
    let t21593 = -t21251 + t21255 - t21317 + t21320 - t21372 + t21263 + t21265 + t21267 - t21270 + t21302 + t21305 - t21336;
    t21593
}
