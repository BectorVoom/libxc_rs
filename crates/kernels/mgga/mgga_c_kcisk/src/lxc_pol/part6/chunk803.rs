//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 803/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk803<F: Float>(t11285: F, t2487: F, t8510: F, t28377: F, t7000: F, t1421: F, t22942: F, t2399: F, t28911: F, t28915: F, t28919: F, t28925: F, t28929: F, t28933: F, t28937: F, t8616: F) -> (F,) {
    let t28941 = t11285 * t8510 * t2487;
    let t28944 = t7000 * t28377;
    let t28948 = 0.65704296666666666667e-3 * t1421 * t28911 - 0.22175200125e-2 * t1421 * t28915 + 0.22175200125e-2 * t1421 * t28919 - 12.0 * t2399 * t8616 + 0.295669335e-2 * t1421 * t28925 - 0.19711289e-2 * t1421 * t28929 - 0.19711289e-2 * t1421 * t28933 + 0.49278222499999999999e-2 * t1421 * t28937 - 0.32852148333333333333e-2 * t1421 * t28941 + 0.32852148333333333333e-2 * t1421 * t28944 + 0.39422577999999999999e-2 * t22942;
    (t28948,)
}
