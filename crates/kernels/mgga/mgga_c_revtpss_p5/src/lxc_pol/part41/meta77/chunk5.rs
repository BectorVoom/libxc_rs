//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 463/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk463<F: Float>(t1570: F, t1580: F, t213: F, t783: F, t791: F, t865: F, t1524: F, t1533: F, t1536: F, t1544: F, t198: F, t207: F, t679: F, t704: F, t751: F, t759: F, t764: F, t765: F, t892: F) -> (F, F) {
    let t1583 = -t783 + t791 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t1570 - F::cast_from(0.65854491829355115987e0_f64) * t865 * t1580;
    let t1587 = t1583 * t198 * t207 * t892 + F::cast_from(3.0_f64) * t1544 * t198 * t765 + t1524 + t1533 + t1536 + t679 + t704 + t751 - t759 - t764;
    (t1583, t1587)
}
