//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1081/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1081<F: Float>(t27716: F, t449: F, t446: F, t448: F, t4504: F, t2233: F, t2272: F, t3708: F, t1300: F, t8014: F, t2167: F, t4527: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27717 = t449 * t27716;
    let t27718 = t446 * t27717;
    let t27719 = t27718 / F::cast_from(16.0_f64);
    let t27720 = t448 * t4504;
    let t27721 = t2233 * t27720;
    let t27722 = t27721 / F::cast_from(16.0_f64);
    let t27723 = t3708 * t2272;
    let t27724 = t446 * t27723;
    let t27725 = t27724 / F::cast_from(16.0_f64);
    let t27726 = t1300 * t8014;
    let t27727 = t446 * t27726;
    let t27728 = t27727 / F::cast_from(8.0_f64);
    let t27733 = t4527 * t2167;
    (t27717, t27719, t27720, t27722, t27723, t27725, t27726, t27728, t27733)
}
