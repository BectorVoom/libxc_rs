//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1108/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1108<F: Float>(t1092: F, t1773: F, t26760: F, t43053: F, t1121: F, t6704: F, t28093: F, t28190: F, t1133: F, t67493: F, t7718: F, t69560: F, t1262: F, t6272: F, t1267: F, t5310: F, t92651: F) -> (F, F, F, F, F, F, F, F) {
    let t100297 = t1092 * t26760 * t43053 * t1773;
    let t100301 = t1092 * t26760 * t6704 * t1121;
    let t100303 = t28190 * t28093;
    let t100307 = t1092 * t7718 * t67493 * t1133;
    let t100312 = t1092 * t26760 * t69560 * t1133;
    let t100314 = t6272 * t1262;
    let t100319 = t6272 * t1267;
    let t100321 = t5310 * t92651 * t100319;
    (t100297, t100301, t100303, t100307, t100312, t100314, t100319, t100321)
}
