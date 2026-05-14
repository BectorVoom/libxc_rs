//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 410/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk410<F: Float>(t123: F, t5241: F, t1980: F, t2032: F, t2084: F, t296: F, t120: F, t19: F, t320: F, t2088: F, t298: F, t2102: F, t769: F, t325: F, t539: F, t107: F) -> (F, F, F, F, F, F, F, F) {
    let t5641 = t5241 * t123;
    let t5676 = t1980 * t2032;
    let t5745 = t2084 * t296;
    let t5746 = t120 * t5745;
    let t5747 = t5746 * t19;
    let t5748 = t320 * t5747;
    let t5750 = 1.0 / t2088 / t298;
    let t5771 = t769 * t2102;
    let t5774 = t539 * t325;
    let t5775 = t107 * t5774;
    (t5641, t5676, t5745, t5747, t5748, t5750, t5771, t5775)
}
