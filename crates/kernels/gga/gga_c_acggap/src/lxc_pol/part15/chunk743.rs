//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 743/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk743<F: Float>(t2176: F, t880: F, t639: F, t7924: F, t2217: F, t309: F, t2132: F, t2131: F, t633: F, t847: F, t851: F, t323: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8096 = F::new(0.65854491829355115987e0) * t2176 * t880;
    let t8098 = F::new(0.8673628188205199462e0) * t7924 * t639;
    let t8099 = t2217 * t309;
    let t8100 = t2132 * t8099;
    let t8101 = t2131 * t8100;
    let t8103 = t633 * t847;
    let t8104 = t2132 * t8103;
    let t8106 = F::new(0.8673628188205199462e0) * t2131 * t8104;
    let t8111 = t851 * t633;
    let t8113 = F::new(0.13170898365871023197e1) * t8111 * t323;
    (t8096, t8098, t8099, t8100, t8101, t8103, t8104, t8106, t8111, t8113)
}
