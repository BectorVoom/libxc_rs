//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 581/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk581<F: Float>(t678: F, t7939: F, t7210: F, t7213: F, t7245: F, t7270: F, t7280: F, t7289: F, t4616: F, t698: F) -> (F, F, F, F, F, F, F, F) {
    let t7940 = t7939 * t678;
    let t8019 = F::new(0.162600798888400151e-2) * t7210;
    let t8020 = F::new(0.162600798888400151e-2) * t7213;
    let t8026 = F::new(0.39726959900411316772e-4) * t7245;
    let t8036 = F::new(0.36366215538993788974e-1) * t7270;
    let t8038 = F::new(0.1454648621559751559e0) * t7280;
    let t8040 = F::new(0.10909864661698136692e0) * t7289;
    let t8041 = t4616 * t698;
    (t7940, t8019, t8020, t8026, t8036, t8038, t8040, t8041)
}
