//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 471/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk471<F: Float>(t4834: F, t606: F, t353: F, t579: F, t964: F, t163: F, t657: F) -> (F, F, F, F, F, F) {
    let t4868 = 4.0 / 9.0 * t4834;
    let t4876 = 0.39862222222222222223e0 * t4834;
    let t4881 = 1.0/f64::sqrt(t606);
    let t4887 = t353 * t964 * t579;
    let t4888 = 0.27385555555555555555e0 * t4887;
    let t4889 = t163 * t657;
    (t4868, t4876, t4881, t4887, t4888, t4889)
}
