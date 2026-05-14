//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 991/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk991<F: Float>(t1507: F, t2020: F, t30120: F, t8793: F, t1165: F, t33735: F, t604: F, t7413: F, t8948: F, t4680: F, t8947: F, t1181: F, t2068: F, t20972: F, t21128: F, t7839: F, t8787: F) -> (F, F, F, F, F, F, F, F) {
    let t36151 = t2020 * t1507;
    let t36156 = t30120 * t8793;
    let t36160 = t7413 * t1165 * t604 * t33735;
    let t36162 = t30120 * t8948;
    let t36165 = t7413 * t4680 * t8947;
    let t36169 = t2068 * t1181 * t604 * t20972;
    let t36173 = t2068 * t1181 * t604 * t21128;
    let t36175 = t7839 * t8787;
    (t36151, t36156, t36160, t36162, t36165, t36169, t36173, t36175)
}
