//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 944/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk944(t24774: f64, t2604: f64, t2609: f64, t8607: f64, t2394: f64, t24747: f64, t1685: f64, t28341: f64, t4790: f64, t28507: f64, t10699: f64, t2605: f64, t9124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
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
