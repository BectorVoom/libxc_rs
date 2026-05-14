//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1087/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1087<F: Float>(t5577: F, t5580: F, t92433: F, t17839: F, t5566: F, t7837: F, t373: F, t5546: F, t172: F, t22626: F, t22628: F, t22623: F, t8042: F, t92356: F, t1614: F, t391: F) -> (F, F, F, F, F, F, F, F) {
    let t92435 = t5577 * t92433 * t5580;
    let t92439 = t5566 * t17839;
    let t92440 = t7837 * t92439;
    let t92441 = t5546 * t373;
    let t92447 = t22626 * t172 * t22628;
    let t92448 = t22623 * t92447;
    let t92456 = t8042 * t92356;
    let t92461 = t1614 * t391;
    (t92435, t92439, t92440, t92441, t92447, t92448, t92456, t92461)
}
