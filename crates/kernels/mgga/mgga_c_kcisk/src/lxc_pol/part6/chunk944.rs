//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 944/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk944<F: Float>(t24774: F, t2604: F, t2609: F, t8607: F, t2394: F, t24747: F, t1685: F, t28341: F, t4790: F, t28507: F, t10699: F, t2605: F, t9124: F) -> (F, F, F, F, F, F, F, F) {
    let t29689 = t24774 * t2604;
    let t29692 = t2609 * t8607;
    let t29695 = t24747 * t2394;
    let t29700 = t28341 * t1685;
    let t29709 = t28341 * t4790;
    let t29712 = t28507 * t1685;
    let t29715 = t28341 * t10699;
    let t29718 = t2605 * t9124;
    (t29689, t29692, t29695, t29700, t29709, t29712, t29715, t29718)
}
