//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 824/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk824<F: Float>(t3217: F, t982: F, t1130: F, t2865: F, t1014: F, t3241: F, t3238: F, t4585: F, t85: F, t349: F, t1098: F, t3290: F) -> (F, F, F, F, F, F, F) {
    let t10245 = t982 * t3217;
    let t10250 = t2865 * t1130;
    let t10255 = t1014 * t3241;
    let t10257 = t1014 * t3238;
    let t10269 = t85 * t4585;
    let t10271 = F::new(0.29201909629629629629e-3) * t10269 * t349;
    let t10282 = t1098 * t3290;
    (t10245, t10250, t10255, t10257, t10269, t10271, t10282)
}
