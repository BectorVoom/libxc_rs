//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 979/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk979<F: Float>(t4975: F, t7561: F, t5157: F, t1165: F, t22401: F, t7351: F, t7413: F, t30817: F, t8948: F, t8793: F, t4434: F, t570: F, t1313: F, t30598: F, t721: F, t1322: F, t7859: F) -> (F, F, F, F, F, F, F, F) {
    let t35866 = t7561 * t4975;
    let t35868 = t7561 * t5157;
    let t35872 = t7413 * t1165 * t7351 * t22401;
    let t35874 = t30817 * t8948;
    let t35876 = t30817 * t8793;
    let t35879 = t570 * t4434;
    let t35882 = t30598 * t1313 * t721;
    let t35885 = t7859 * t1322 * t721;
    (t35866, t35868, t35872, t35874, t35876, t35879, t35882, t35885)
}
