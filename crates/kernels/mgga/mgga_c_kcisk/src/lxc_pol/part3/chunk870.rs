//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 870/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk870<F: Float>(t12885: F, t13064: F, t3725: F, t12916: F, t12919: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12935: F, t12937: F, t12939: F, t12943: F, t12946: F, t12948: F, t12954: F) -> (F, F) {
    let t13066 = t13064 * t12885 * t3725;
    let t13083 = -F::new(0.3883875e1) * t12916 + F::new(0.247573125e0) * t12919 - F::new(0.33547222222222222222e0) * t12922 - F::new(0.301925e0) * t12927 - F::new(0.40256666666666666668e0) * t12929 + F::new(0.30192500000000000001e0) * t12931 + F::new(0.20128333333333333333e0) * t12933 - F::new(0.27595e0) * t12935 + F::new(0.16557e0) * t12937 + F::new(0.5519e-1) * t12939 - F::new(0.36793333333333333333e-1) * t12943 - F::new(0.82785e-1) * t12946 - F::new(0.60385000000000000001e0) * t12948 + F::new(0.12077e1) * t12954;
    (t13066, t13083)
}
