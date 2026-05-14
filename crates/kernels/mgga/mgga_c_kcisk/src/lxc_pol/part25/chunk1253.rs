//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1253/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1253<F: Float>(t1065: F, t32683: F, t3299: F, t9406: F, t32684: F, t32553: F, t32582: F, t32588: F, t32556: F, t5217: F, t9694: F, t11798: F, t654: F, t11730: F, t33120: F, t4816: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t111533 = t1065 * t32683;
    let t111564 = t3299 * t9406;
    let t111577 = 3.0 * t32684;
    let t111582 = 6.0 * t32553;
    let t111583 = 18.0 * t32582;
    let t111584 = 3.0 * t32588;
    let t111585 = 6.0 * t32556;
    let t112011 = t9694 * t5217;
    let t112046 = t11798 * t654;
    let t112051 = t11730 * t654;
    let t112095 = t4816 * t33120;
    (t111533, t111564, t111577, t111582, t111583, t111584, t111585, t112011, t112046, t112051, t112095)
}
