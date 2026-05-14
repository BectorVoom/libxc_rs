//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 464/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk464<F: Float>(t3208: F, t5: F, t1016: F, t119: F, t4: F, t181: F, t944: F, t3088: F, t1021: F, t3107: F, t1011: F, t1015: F, t1018: F, t1022: F, t11: F, t139: F, t157: F, t175: F, t197: F, t198: F, t201: F, t3125: F, t3190: F, t3194: F, t3200: F, t3203: F, t3207: F, t972: F) -> (F, F, F, F, F, F) {
    let t3209 = t5 * t3208;
    let t3213 = t1016 * t4 * t119;
    let t3216 = t181 * t944;
    let t3217 = t3216 * t3088;
    let t3220 = t1021 * t3107;
    let t3232 = 0.619125e-2 * t3190 * t198 - 0.24765e-1 * t3194 * t1018 - 0.123825e-1 * t1011 * t1022 + 0.206375e-2 * t3200 * t3203 + 0.24765e-1 * t3207 * t3209 + 0.1651e-1 * t1015 * t3213 + 0.123825e-1 * t197 * t3217 - 0.619125e-2 * t197 * t3220 + 0.17687407407407407407e-1 * t139 * t157 * t175 - 0.10612444444444444444e0 * t139 * t11 * t972 - 0.79593333333333333331e-1 * t139 * t201 * t3125;
    (t3209, t3213, t3216, t3217, t3220, t3232)
}
