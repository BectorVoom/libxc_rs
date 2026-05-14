//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 958/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk958<F: Float>(t19424: F, t19450: F, t4475: F, t5770: F, t4472: F, t5793: F, t3722: F, t5788: F, t5794: F, t2079: F, t3676: F, t3680: F, t12900: F, t2093: F, t3634: F, t5749: F) -> (F, F, F, F, F, F, F) {
    let t19451 = t19450 * t19424;
    let t19456 = t5770 * t4475;
    let t19459 = t5793 * t4472;
    let t19472 = t3722 * t5788;
    let t19473 = t19472 * t5794;
    let t19476 = t2079 * t3676;
    let t19478 = 0.16081824322151104822e2 * t19476 * t3680;
    let t19480 = 1.0 * t12900 * t2093;
    let t19482 = 2.0 * t3634 * t5749;
    (t19451, t19456, t19459, t19473, t19478, t19480, t19482)
}
