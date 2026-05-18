//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 958/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk958<F: Float>(t16060: F, t5968: F, t1317: F, t1507: F, t17636: F, t5463: F, t1517: F, t167: F, t4225: F, t2026: F, t752: F, t3393: F, t5973: F) -> (F, F, F, F, F, F) {
    let t17649 = t5968 * t16060;
    let t17656 = t1507 * t1317;
    let t17669 = t5463 * t17636;
    let t17673 = t1517 * t4225 * t167;
    let t17676 = t752 * t2026;
    let t17685 = F::new(0.35374814814814814814e-1) * t3393 * t5973;
    (t17649, t17656, t17669, t17673, t17676, t17685)
}
