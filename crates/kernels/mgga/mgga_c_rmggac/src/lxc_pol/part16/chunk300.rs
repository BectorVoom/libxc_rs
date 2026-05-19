//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 300/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk300<F: Float>(t53: F, t60: F, t1794: F, t1797: F, t57: F, t912: F, t525: F, t62: F, t921: F, zeta_threshold: F) -> (F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1801 = piecewise3::<F>(t54, F::new(0.0), F::new(4.0) / F::new(9.0) * t912 * t1794 + F::new(4.0) / F::new(3.0) * t57 * t1797);
    let t1802 = t525 * t525;
    let t1805 = -t1797;
    let t1809 = piecewise3::<F>(t61, F::new(0.0), F::new(4.0) / F::new(9.0) * t921 * t1802 + F::new(4.0) / F::new(3.0) * t62 * t1805);
    let t1810 = t1801 + t1809;
    (t1802, t1805, t1810)
}
