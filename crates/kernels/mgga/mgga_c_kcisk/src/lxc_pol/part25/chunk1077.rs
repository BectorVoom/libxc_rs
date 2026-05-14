//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1077/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1077<F: Float>(t1139: F, t9390: F, t31825: F, t31827: F, t31829: F, t31832: F, t31835: F, t31838: F, t31840: F, t31842: F, t31844: F, t31846: F, t31849: F, t31852: F, t31855: F, t31858: F) -> (F, F) {
    let t32560 = t9390 * t1139;
    let t32579 = 0.1875e0 * t31825 - 0.375e0 * t31827 - 0.75e0 * t31829 + 0.375e0 * t31832 + 0.75e0 * t31835 - 0.1875e0 * t31838 + 0.1125e1 * t31840 - 0.809375e-1 * t31842 + 0.161875e0 * t31844 + 0.6475e0 * t31846 - 0.161875e0 * t31849 - 0.6475e0 * t31852 + 0.809375e-1 * t31855 - 0.161875e1 * t31858;
    (t32560, t32579)
}
