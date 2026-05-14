//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1163/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1163<F: Float>(t34016: F, t6667: F, t7234: F, t1791: F, t2464: F, t1636: F, t5015: F, t7283: F, t7242: F) -> (F, F, F, F, F, F) {
    let t34017 = t34016 * t6667;
    let t34018 = t7234 * t34017;
    let t34021 = t1791 * t2464;
    let t34022 = t34021 * t1636;
    let t34023 = t5015 * t34022;
    let t34026 = t7283 * t1636;
    let t34027 = t7242 * t34026;
    (t34017, t34018, t34022, t34023, t34026, t34027)
}
