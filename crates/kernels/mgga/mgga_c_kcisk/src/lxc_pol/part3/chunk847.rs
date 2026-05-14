//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 847/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk847<F: Float>(t3866: F, t970: F, t3870: F, t12925: F, t1398: F, t12831: F, t457: F, t3875: F, t960: F, t3878: F, t1375: F, t1471: F, t3883: F, t965: F, t13987: F, t13989: F, t158: F, t173: F) -> (F,) {
    let t13991 = t970 * t3866;
    let t13993 = t970 * t3870;
    let t13995 = t1398 * t12925;
    let t13998 = t457 * t12831;
    let t14001 = t960 * t3875;
    let t14003 = t960 * t3878;
    let t14005 = t1375 * t12925;
    let t14008 = t1471 * t12831;
    let t14011 = t965 * t3883;
    let t14013 = -0.28104e-1 * t13987 - 0.32788e-1 * t13989 - 0.352891875e-4 * t13991 + 0.4705225e-4 * t13993 + 0.50413125e-5 * t173 * t13995 + 0.22405833333333333333e-5 * t173 * t13998 + 0.14052e-1 * t14001 - 0.4684e-2 * t14003 - 0.3513e-2 * t158 * t14005 + 0.78066666666666666667e-3 * t158 * t14008 - 0.39624999999999999999e-2 * t14011;
    (t14013,)
}
