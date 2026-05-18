//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 729/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk729<F: Float>(t11287: F, t11290: F, t11294: F, t11298: F, t11302: F, t11306: F, t11309: F, t11314: F, t11316: F, t11318: F, t11320: F, t1421: F) -> F {
    let t11322 = -F::new(0.32852148333333333333e-2) * t1421 * t11287 + F::new(0.32852148333333333333e-2) * t1421 * t11290 + F::new(0.295669335e-2) * t1421 * t11294 + F::new(0.295669335e-2) * t1421 * t11298 - F::new(0.19711289e-2) * t1421 * t11302 - F::new(0.19711289e-2) * t1421 * t11306 - F::new(0.39422577999999999999e-2) * t1421 * t11309 - F::new(0.43802864444444444445e-3) * t11314 + F::new(0.13140859333333333334e-2) * t11316 + F::new(0.21901432222222222222e-2) * t11318 - F::new(0.59133867e-2) * t11320;
    t11322
}
