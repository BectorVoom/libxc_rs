//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1268/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1268<F: Float>(t30620: F, t30622: F, t30624: F, t30626: F, t30628: F, t30637: F, t30724: F, t30727: F, t30729: F, t30734: F, t30739: F, t30697: F, t30742: F, t30745: F, t30747: F, t30749: F, t30751: F, t30753: F, t30755: F, t30758: F, t30761: F, t30764: F) -> (F, F) {
    let t31005 = t30724 - t30727 - t30729 + t30620 + t30622 + t30624 - t30626 - t30628 - t30637 - t30734 - t30739;
    let t31007 = -t30742 + t30745 - t30747 - t30749 - t30751 - t30753 - t30755 + t30697 - t30758 + t30761 + t30764;
    (t31005, t31007)
}
