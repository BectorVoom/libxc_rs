//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1065/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1065<F: Float>(t34620: F, t30937: F, t8602: F, t1165: F, t4718: F, t7351: F, t7426: F, t1181: F, t4818: F, t599: F, t8463: F, t30543: F, t8469: F) -> (F, F, F, F, F) {
    let t34621 = F::new(0.18868855373762491241e-2) * t34620;
    let t34622 = t30937 * t8602;
    let t34623 = F::new(0.37737710747524982482e-2) * t34622;
    let t34626 = t7426 * t1165 * t7351 * t4718;
    let t34627 = F::new(0.94344276868812456204e-3) * t34626;
    let t34630 = t8463 * t1181 * t599 * t4818;
    let t34632 = t30543 * t8469;
    (t34621, t34623, t34627, t34630, t34632)
}
