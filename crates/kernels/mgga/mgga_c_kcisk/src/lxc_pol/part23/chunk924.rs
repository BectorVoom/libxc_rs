//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 924/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk924<F: Float>(t19087: F, t5634: F, t3484: F, t19086: F, t13325: F, t13344: F, t19056: F, t19060: F, t19064: F, t19070: F, t19073: F, t19076: F, t19079: F, t19082: F, t2076: F, t2869: F) -> (F, F, F, F) {
    let t19088 = t5634 * t19087;
    let t19089 = t3484 * t19088;
    let t19090 = t19086 * t19089;
    let t19093 = -0.33163888888888888888e-2 * t19056 + 0.11054629629629629629e-2 * t19060 + 0.18424382716049382715e-2 * t19064 + 0.11054629629629629629e-2 * t19070 - 0.58958024691358024689e-2 * t19073 + t19076 + 0.33163888888888888888e-2 * t19079 - 0.88437037037037037034e-2 * t19082 - 0.3684876543209876543e-3 * t13325 + 0.11054629629629629629e-1 * t19090 - 0.22109259259259259258e-2 * t13344;
    let t19100 = t2869 * t2076;
    (t19088, t19090, t19093, t19100)
}
