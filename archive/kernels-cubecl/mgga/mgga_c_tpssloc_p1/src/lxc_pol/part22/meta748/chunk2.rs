//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2502/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2502<F: Float>(t18915: F, t4875: F, t1155: F, t1694: F, t18615: F, t51848: F, t47774: F, t51002: F, t68513: F) -> (F, F, F, F) {
    let t71114 = F::cast_from(0.35089341735807877242e1_f64) * t18915 * t4875;
    let t71115 = t1694 * t1155;
    let t71118 = F::cast_from(0.31168546390226634766e3_f64) * t51848 * t18615 * t71115;
    let t71124 = t47774 * t51002 * t68513;
    (t71114, t71115, t71118, t71124)
}
