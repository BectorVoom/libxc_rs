//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 474/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk474<F: Float>(t1421: F, t338: F, t3519: F, t3522: F, t3524: F, t3526: F, t3535: F, t3541: F, t3546: F, t3551: F, t3555: F, t3561: F, t3567: F, t3589: F, t3595: F, t3621: F, t3624: F, t3729: F, t456: F) -> F {
    let t3732 = -t3519 + F::new(0.8760572888888888889e-3) * t3522 + F::new(0.19711289e-2) * t3524 - F::new(0.13140859333333333333e-2) * t3526 + F::new(0.10950716111111111111e-2) * t1421 * t3535 + F::new(0.19711289e-2) * t1421 * t3541 - F::new(0.13140859333333333333e-2) * t1421 * t3546 - F::new(0.13140859333333333333e-2) * t1421 * t3551 + F::new(0.65704296666666666667e-3) * t1421 * t3555 + F::new(0.7391733375e-3) * t456 * t3561 - F::new(0.295669335e-2) * t1421 * t3567 + F::new(0.1478346675e-2) * t456 * t3589 + F::new(0.19711289e-2) * t456 * t3595 - F::new(0.98556445e-3) * t456 * t3621 - F::new(4.0) * t3624 - F::new(4.0) * t338 * t3729;
    t3732
}
