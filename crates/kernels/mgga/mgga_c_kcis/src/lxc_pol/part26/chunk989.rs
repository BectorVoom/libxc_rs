//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 989/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk989<F: Float>(t1517: F, t1650: F, t17546: F, t12371: F, t6281: F, t4225: F, t6284: F, t1518: F, t18431: F, t21786: F, t509: F, t2018: F, t543: F) -> (F, F, F, F, F, F) {
    let t22547 = t1517 * t17546 * t1650;
    let t22554 = t1517 * t12371 * t6281;
    let t22558 = t1517 * t4225 * t6284;
    let t22562 = t1517 * t1518 * t18431;
    let t22570 = t509 * t21786;
    let t22574 = t2018 * t543;
    (t22547, t22554, t22558, t22562, t22570, t22574)
}
