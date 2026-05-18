//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 944/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk944<F: Float>(t2061: F, t4302: F, t578: F, t16673: F, t4261: F, t4260: F, t4306: F, t16721: F, t4293: F, t6010: F, t4281: F, t5929: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17427 = t2061 * t4302;
    let t17428 = t578 * t17427;
    let t17430 = t4261 * t16673;
    let t17431 = t4260 * t17430;
    let t17433 = t2061 * t4306;
    let t17434 = t578 * t17433;
    let t17436 = t4293 * t16721;
    let t17437 = t6010 * t17436;
    let t17439 = t4281 * t5929;
    (t17427, t17428, t17430, t17431, t17433, t17434, t17436, t17437, t17439)
}
