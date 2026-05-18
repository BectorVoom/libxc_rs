//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 961/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk961<F: Float>(t10998: F, t11189: F, t3275: F, t1149: F, t1353: F, t10940: F, t3469: F, t10626: F, t3465: F, t3492: F, t498: F) -> (F, F, F, F, F) {
    let t11191 = t3275 * t11189 * t10998;
    let t11192 = F::new(45.0) / F::new(64.0) * t11191;
    let t11193 = t1353 * t1149;
    let t11194 = t10940 * t3469;
    let t11195 = t11194 / F::new(4.0);
    let t11197 = t3275 * t3465 * t10626;
    let t11198 = t11197 / F::new(2.0);
    let t11199 = t498 * t3492;
    (t11192, t11193, t11195, t11198, t11199)
}
