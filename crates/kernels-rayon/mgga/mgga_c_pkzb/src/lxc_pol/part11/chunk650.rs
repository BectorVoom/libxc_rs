//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 650/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk650(t2127: f64, t3679: f64, t133: f64, t3650: f64, t793: f64, t2139: f64, t1138: f64, t2123: f64, t2138: f64, t290: f64, t2984: f64, t3669: f64, t791: f64) -> (f64, f64, f64, f64, f64) {
    let t3680 = t3679 * t2127;
    let t3685 = t3650 * t133;
    let t3686 = t3685 * t793;
    let t3689 = t3679 * t2139;
    let t3694 = 0.13170898365871023197e1_f64 * t2123 * t3680 + 0.13170898365871023197e1_f64 * t2984 * t1138 + 0.65854491829355115987e0_f64 * t791 * t3686 - 0.65854491829355115987e0_f64 * t2138 * t3689 + 0.65854491829355115987e0_f64 * t290 * t3669;
    (t3680, t3685, t3686, t3689, t3694)
}
