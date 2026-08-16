//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1128/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1128<F: Float>(t14722: F, t14704: F, t11147: F, t1409: F, t2244: F, t11145: F, t123: F) -> (F, F, F, F) {
    let t14723 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14722;
    let t14724 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14704;
    let t14725 = t11147 * t1409;
    let t14726 = t14725 * t2244;
    let t14727 = t11145 * t14726;
    let t14728 = t123 * t14727;
    (t14723, t14724, t14726, t14728)
}
