//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1093/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1093<F: Float>(t2790: F, t3805: F, t2789: F, t5175: F, t415: F, t5074: F, t9681: F, t1944: F, t654: F) -> (F, F, F, F, F, F) {
    let t32896 = t3805 * t2790;
    let t32897 = 0.55273148148148148147e-3 * t32896;
    let t32898 = t5175 * t2789;
    let t32899 = t415 * t32898;
    let t32901 = t5074 * t9681;
    let t32903 = t1944 * t654;
    (t32896, t32897, t32898, t32899, t32901, t32903)
}
