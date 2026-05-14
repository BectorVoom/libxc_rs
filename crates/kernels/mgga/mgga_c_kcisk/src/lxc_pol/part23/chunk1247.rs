//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1247/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1247<F: Float>(t32155: F, t32157: F, t32433: F, t32477: F, t33548: F, t33559: F, t33562: F, t33564: F, t33571: F, t33578: F, t33784: F, t33937: F, t33941: F, t9539: F, t9855: F, t394: F, t6448: F) -> (F, F) {
    let t33953 = 0.19345601851851851852e-2 * t33548 - 0.17411041666666666666e-2 * t33559 - 0.116403125e-2 * t33937 * t33784 - 0.17361111111111111111e-2 * t33941 * t9539 - 0.46429444444444444443e-2 * t33562 + 0.11607361111111111111e-2 * t33564 - 0.17361111111111111111e-2 * t32477 - 0.53611111111111111112e-2 * t32433 * t9855 - 0.11607361111111111111e-2 * t32155 + 0.77382407407407407407e-3 * t32157 - 0.11607361111111111111e-2 * t33571 - 0.17411041666666666666e-2 * t33578;
    let t33959 = t6448 * t394;
    (t33953, t33959)
}
