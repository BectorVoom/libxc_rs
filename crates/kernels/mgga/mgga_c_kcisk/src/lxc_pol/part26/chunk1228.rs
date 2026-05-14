//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1228/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1228<F: Float>(t21499: F, t32101: F, t13900: F, t9446: F, t9448: F, t3969: F, t32065: F, t53214: F, t9428: F, t9426: F, t110077: F, t10349: F, t31883: F, t15461: F, t9358: F, t3441: F, t9390: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t110635 = t32101 * t21499;
    let t110648 = t9446 * t13900 * t9448;
    let t110655 = t32101 * t3969;
    let t110663 = t32065 * t21499;
    let t110691 = t53214 * t9428;
    let t110692 = t9446 * t110691;
    let t110695 = t9426 * t110691;
    let t110762 = 0.73697530864197530862e-3 * t110077;
    let t110815 = 6.0 * t31883 * t10349;
    let t110817 = 3.0 * t15461 * t9358;
    let t110821 = t9390 * t3441;
    (t110635, t110648, t110655, t110663, t110692, t110695, t110762, t110815, t110817, t110821)
}
