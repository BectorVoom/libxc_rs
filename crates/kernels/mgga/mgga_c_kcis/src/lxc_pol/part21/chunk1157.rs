//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1157/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1157<F: Float>(t28093: F, t7772: F, t1268: F, t1851: F, t922: F, t3515: F, t5281: F, t5310: F) -> (F, F, F, F, F, F) {
    let t28094 = t7772 * t28093;
    let t28096 = t1268 * t1851;
    let t28097 = t28096 * t922;
    let t28098 = t3515 * t28097;
    let t28101 = t5281 * t922;
    let t28102 = t5310 * t28101;
    (t28094, t28096, t28097, t28098, t28101, t28102)
}
