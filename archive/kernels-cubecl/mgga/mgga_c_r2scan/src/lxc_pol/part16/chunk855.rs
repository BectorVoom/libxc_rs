//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 855/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk855<F: Float>(t2060: F, t3033: F, t2062: F, t2823: F, t7902: F, t4695: F, t4703: F, t4880: F, t4891: F, t6946: F, t6948: F, t6951: F, t8545: F, t8547: F) -> F {
    let t9033 = t2060 * t3033;
    let t9034 = t9033 * t2062;
    let t9036 = t2823 * t7902;
    let t9038 = -t4695 - t4880 + t6946 - t8545 - F::cast_from(0.675260332e-1_f64) * t9034 - F::cast_from(0.1350520664e0_f64) * t9036 + t6948 + t4891 - t4703 - t6951 - t8547;
    t9038
}
