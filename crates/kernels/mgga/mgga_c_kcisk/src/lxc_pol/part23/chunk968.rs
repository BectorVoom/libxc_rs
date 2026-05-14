//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 968/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk968<F: Float>(t12931: F, t12933: F, t12935: F, t12937: F, t12939: F, t12948: F, t19100: F, t19106: F, t19116: F, t19121: F, t19125: F, t19129: F, t19485: F, t19488: F, t19491: F, t19494: F, t19497: F, t19528: F, t19645: F, t19667: F, t19678: F, t19689: F) -> (F,) {
    let t19691 = -t19645 + 0.36793333333333333334e-1 * t19485 - 0.27595e-1 * t19488 + 0.16557e0 * t19491 - 0.49671e0 * t19494 - 0.36793333333333333333e-1 * t19497 + 0.12077e1 * t19116 - 0.80513333333333333333e0 * t19121 - 0.20128333333333333333e0 * t19125 - 0.181155e1 * t19129 + t19667 + 0.82524375e-1 * t19528 - 0.13418888888888888889e0 * t19100 + 0.22141166666666666666e1 * t19106 + 0.10064166666666666667e0 * t12931 + 0.67094444444444444447e-1 * t12933 - 0.18396666666666666667e0 * t12935 + 0.5519e-1 * t12937 + 0.18396666666666666667e-1 * t12939 - 0.20128333333333333334e0 * t12948 + t19678 + t19689;
    (t19691,)
}
