//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1375/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1375<F: Float>(t10265: F, t159: F, t585: F, t617: F, t10245: F, t750: F, t741: F, t21743: F, t22187: F, t22191: F, t22194: F, t22196: F, t22206: F, t26596: F, t26597: F, t26599: F, t26602: F, t26603: F) -> (F,) {
    let t33554 = t159 * t10265 * t585 * t617;
    let t33556 = t10245 * t750;
    let t33558 = t10245 * t741;
    let t33564 = -0.12304822629859687989e5 * t22187 - t22191 - t22194 + 0.84681398666666666666e-3 * t33554 + 0.17315859105681463759e2 * t33556 - 0.11696447245269292414e1 * t33558 - 0.571528e-1 * t22196 - t22206 - t21743 - t26596 - 0.93505639170679904295e3 * t26597 - 0.36914467889579063967e5 * t26599 + t26602 - 0.35089341735807877242e1 * t26603;
    (t33564,)
}
