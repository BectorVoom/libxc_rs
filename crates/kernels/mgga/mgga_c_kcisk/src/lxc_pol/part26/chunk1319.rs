//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1319/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1319<F: Float>(t113599: F, t1339: F, t9814: F, t33366: F, t33604: F, t5600: F, t32203: F, t34815: F, t25312: F, t3797: F, t9461: F, t34700: F, t3759: F, t34724: F, t3739: F, t34810: F, t3748: F) -> (F, F, F, F, F, F, F) {
    let t119066 = t1339 * t113599 * t9814;
    let t119069 = t5600 * t33604 * t33366;
    let t119072 = t1339 * t32203 * t34815;
    let t119076 = t1339 * t9461 * t3797 * t25312;
    let t119079 = t3759 * t32203 * t34700;
    let t119083 = t3739 * t34724;
    let t119088 = t3748 * t34810;
    (t119066, t119069, t119072, t119076, t119079, t119083, t119088)
}
