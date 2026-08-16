//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1074/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1074<F: Float>(t28040: F, t28070: F, t1142: F, t1291: F, t8117: F, t1872: F, t7823: F, t15573: F, t8086: F) -> (F, F, F, F, F) {
    let t28071 = t28040 + t28070;
    let t28072 = t1142 * t28071;
    let t28073 = t8117 * t1291;
    let t28076 = t7823 * t1872;
    let t28093 = t15573 * t8086;
    (t28071, t28072, t28073, t28076, t28093)
}
