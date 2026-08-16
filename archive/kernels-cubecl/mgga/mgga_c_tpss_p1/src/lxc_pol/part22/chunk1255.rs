//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1255/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1255<F: Float>(t1656: F, t5918: F, t5740: F, t1838: F, t4516: F, t18967: F, t19535: F, t3255: F) -> (F, F, F, F, F, F) {
    let t20178 = t5918 * t1656;
    let t20179 = t5740 * t20178;
    let t20182 = t1838 * t4516;
    let t20183 = t5740 * t20182;
    let t20187 = t18967 * t19535;
    let t20190 = t3255 * t1838;
    (t20178, t20179, t20182, t20183, t20187, t20190)
}
