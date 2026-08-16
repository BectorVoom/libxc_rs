//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1268/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1268(t30620: f64, t30622: f64, t30624: f64, t30626: f64, t30628: f64, t30637: f64, t30724: f64, t30727: f64, t30729: f64, t30734: f64, t30739: f64, t30697: f64, t30742: f64, t30745: f64, t30747: f64, t30749: f64, t30751: f64, t30753: f64, t30755: f64, t30758: f64, t30761: f64, t30764: f64) -> (f64, f64) {
    let t31005 = t30724 - t30727 - t30729 + t30620 + t30622 + t30624 - t30626 - t30628 - t30637 - t30734 - t30739;
    let t31007 = -t30742 + t30745 - t30747 - t30749 - t30751 - t30753 - t30755 + t30697 - t30758 + t30761 + t30764;
    (t31005, t31007)
}
