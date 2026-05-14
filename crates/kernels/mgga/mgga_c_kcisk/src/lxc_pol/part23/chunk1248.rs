//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1248/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1248<F: Float>(t20: F, t33959: F, t2734: F, t2737: F, t33873: F, t2740: F, t32187: F, t32201: F, t32502: F, t33585: F, t33598: F, t33602: F, t33606: F, t33611: F, t33823: F, t9529: F, t9855: F) -> (F, F, F) {
    let t33960 = t33959 * t20;
    let t33961 = t2734 * t33960;
    let t33966 = t2737 * t33873;
    let t33972 = -t32502 - 0.30952962962962962963e-2 * t32187 + 0.52083333333333333333e-2 * t2737 * t33823 + 0.77382407407407407407e-3 * t32201 + 0.17411041666666666666e-2 * t33585 - 0.52083333333333333333e-2 * t33961 * t2740 - 0.13888888888888888889e-1 * t9529 * t9855 + 0.17361111111111111111e-2 * t33966 - 0.11607361111111111111e-2 * t33598 - 0.38691203703703703703e-3 * t33602 + 0.11607361111111111111e-2 * t33606 + 0.34822083333333333332e-2 * t33611;
    (t33960, t33961, t33972)
}
