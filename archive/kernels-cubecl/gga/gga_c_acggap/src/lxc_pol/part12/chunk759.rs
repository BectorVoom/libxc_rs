//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 759/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk759<F: Float>(t2132: F, t8099: F, t2131: F, t633: F, t847: F, t1221: F, t8004: F, t851: F, t323: F, t2217: F, t315: F, t943: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8100 = t2132 * t8099;
    let t8101 = t2131 * t8100;
    let t8103 = t633 * t847;
    let t8104 = t2132 * t8103;
    let t8106 = F::cast_from(0.8673628188205199462e0_f64) * t2131 * t8104;
    let t8107 = t633 * t1221;
    let t8108 = t8004 * t8107;
    let t8111 = t851 * t633;
    let t8113 = F::cast_from(0.13170898365871023197e1_f64) * t8111 * t323;
    let t8114 = t315 * t2217;
    let t8115 = t8114 * t323;
    let t8117 = t633 * t943;
    (t8100, t8101, t8103, t8104, t8106, t8107, t8108, t8111, t8113, t8114, t8115, t8117)
}
