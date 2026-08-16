//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3151/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3151<F: Float>(t63290: F, t64477: F, t64479: F, t64481: F, t64485: F, t64489: F, t64492: F, t64496: F, t64499: F, t64501: F, t64504: F, t64507: F, t64509: F) -> F {
    let t65279 = -t64477 - t64479 - t64481 + t64485 + t64489 + t64492 - t64496 - t64499 + t64501 - t63290 - t64504 - t64507 + t64509;
    t65279
}
