//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 940/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk940<F: Float>(t1309: F, t1315: F, t13821: F, t13839: F, t13897: F, t13902: F, t13906: F, t13910: F, t13913: F, t13919: F, t13924: F, t13927: F, t3944: F, t3948: F, t3955: F, t3966: F, t3970: F) -> F {
    let t13932 = F::cast_from(0.71963154864709268852e-1_f64) * t3966 * t3955 + F::cast_from(0.55971342672551653552e-1_f64) * t1309 * t13897 - F::cast_from(0.11993859144118211475e-1_f64) * t13902 - F::cast_from(0.28785261945883707542e0_f64) * t13839 * t1315 + F::cast_from(0.35981577432354634425e-1_f64) * t13906 + F::cast_from(0.52772980234120130492e0_f64) * t13821 * t1315 - F::cast_from(0.95950873152945691802e-1_f64) * t13910 + F::cast_from(0.17990788716177317213e-1_f64) * t13913 - F::cast_from(0.19190174630589138361e0_f64) * t3970 * t3955 + F::cast_from(0.2398771828823642295e-1_f64) * t13919 + F::cast_from(0.28785261945883707541e0_f64) * t3970 * t3944 - F::cast_from(0.35981577432354634425e-1_f64) * t13924 + F::cast_from(0.10794473229706390328e0_f64) * t1309 * t13927 - F::cast_from(0.1439263097294185377e0_f64) * t3970 * t3948;
    t13932
}
