//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1429/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1429<F: Float>(t2562: F, t3016: F, t2148: F, t6165: F, t2155: F, t33868: F, t22950: F, t31069: F, t31072: F, t31075: F, t31083: F, t31086: F, t31092: F, t31095: F, t31099: F, t31102: F, t31117: F, t7313: F, t9409: F) -> (F,) {
    let t34640 = t2562 * t3016;
    let t34642 = t6165 * t2148 * t34640;
    let t34644 = t2155 * t33868;
    let t34650 = 0.29272321618148349056e-1 * t31069 - 0.14636160809074174528e-1 * t31072 - 0.32927245914677557992e-1 * t31075 + 0.52396431978519890151e-1 * t31083 - 0.34930954652346593434e-1 * t31086 + 0.20803732176130244552e1 * t31092 + 0.58544643236296698111e-1 * t31095 - 0.48787202696913915093e-3 * t31099 - 0.1047928639570397803e0 * t31102 + 0.52396431978519890152e-1 * t34642 + 0.8781696485444504717e-1 * t34644 - 0.41530324072742201648e-1 * t22950 + 0.52009330440325611378e0 * t7313 * t9409 - 0.1047928639570397803e0 * t31117;
    (t34650,)
}
