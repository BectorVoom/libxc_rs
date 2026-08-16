//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 913/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk913(t6177: f64, t6218: f64, t7950: f64, t8090: f64, t8091: f64, t9812: f64, t9814: f64, t9819: f64, t9823: f64, t9826: f64, t9830: f64, t9834: f64) -> f64 {
    let t9928 = 0.82524375e-1_f64 * t9812 + 0.16504875e0_f64 * t9814 - t6218 + 0.27595e0_f64 * t6177 + 0.5519e0_f64 * t7950 - t8090 - t8091 - 0.16557e0_f64 * t9819 + 0.49671e0_f64 * t9823 - 0.16557e0_f64 * t9826 + 0.248355e0_f64 * t9830 + 0.248355e0_f64 * t9834;
    t9928
}
