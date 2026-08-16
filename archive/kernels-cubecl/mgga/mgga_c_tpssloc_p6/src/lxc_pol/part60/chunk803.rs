//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 803/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk803<F: Float>(t28951: F, t510: F, t2035: F, t5456: F, t28834: F, t7170: F, t2057: F, t28241: F, t1510: F, t26661: F, t24255: F, t5585: F) -> (F, F, F, F, F, F) {
    let t28952 = t510 * t28951;
    let t28959 = t2035 * t5456;
    let t28969 = t7170 * t28834;
    let t28972 = t2057 * t28241;
    let t28997 = t26661 * t1510;
    let t29000 = t24255 * t5585;
    (t28952, t28959, t28969, t28972, t28997, t29000)
}
