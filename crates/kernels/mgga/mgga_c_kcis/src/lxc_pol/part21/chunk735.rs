//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 735/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk735<F: Float>(t393: F, t8060: F, t1820: F, t7740: F, t2189: F, t5036: F) -> (F, F, F, F) {
    let t8061 = t8060 * t393;
    let t8062 = t7740 * t1820;
    let t8063 = t5036 * t2189;
    let t8064 = t2189 * t1820;
    (t8061, t8062, t8063, t8064)
}
