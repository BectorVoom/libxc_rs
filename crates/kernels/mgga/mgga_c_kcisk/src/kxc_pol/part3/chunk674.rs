//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 674/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk674<F: Float>(t11313: F, t1879: F, t3521: F, t4620: F, t4600: F, t4632: F, t11287: F, t11290: F, t11294: F, t11298: F, t11302: F, t11306: F, t11309: F, t1421: F, t10441: F, t1876: F, t4598: F) -> (F, F) {
    let t11314 = t11313 * t1879;
    let t11316 = t3521 * t4620;
    let t11318 = t3521 * t4600;
    let t11320 = t3521 * t4632;
    let t11322 = -0.32852148333333333333e-2 * t1421 * t11287 + 0.32852148333333333333e-2 * t1421 * t11290 + 0.295669335e-2 * t1421 * t11294 + 0.295669335e-2 * t1421 * t11298 - 0.19711289e-2 * t1421 * t11302 - 0.19711289e-2 * t1421 * t11306 - 0.39422577999999999999e-2 * t1421 * t11309 - 0.43802864444444444445e-3 * t11314 + 0.13140859333333333334e-2 * t11316 + 0.21901432222222222222e-2 * t11318 - 0.59133867e-2 * t11320;
    let t11325 = t1876 * t4598 * t10441;
    (t11322, t11325)
}
