//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 468/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk468<F: Float>(t53: F, t1794: F, t3985: F, t1797: F, t912: F, t3878: F, t814: F, t1395: F, t280: F, t57: F, t815: F, t1802: F, t3998: F, zeta_threshold: F) -> (F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t5850 = t3985 * t1794;
    let t5855 = t912 * t1797;
    let t5860 = -F::new(2.0) * t814 - F::new(6.0) * t3878;
    let t5864 = piecewise3::<F>(t54, F::new(0.0), -F::new(8.0) / F::new(27.0) * t5850 * t280 + F::new(16.0) / F::new(9.0) * t1395 * t815 + F::new(4.0) / F::new(9.0) * t5855 * t280 + F::new(4.0) / F::new(3.0) * t57 * t5860);
    let t5865 = t3998 * t1802;
    (t5860, t5864, t5865)
}
