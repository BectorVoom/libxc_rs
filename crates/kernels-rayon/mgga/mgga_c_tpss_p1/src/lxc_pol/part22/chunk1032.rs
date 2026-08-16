//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1032/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1032(t11144: f64, t285: f64, t3907: f64, t8833: f64, t912: f64, t2593: f64, t3882: f64, t905: f64, t1448: f64, t8749: f64, t2595: f64, t8752: f64) -> (f64, f64, f64, f64, f64) {
    let t11146 = 0.621814e-1_f64 * t11144 * t285;
    let t11147 = t3907 * t8833;
    let t11149 = 0.17315859105681463759e2_f64 * t912 * t11147;
    let t11152 = t2593 * t3882;
    let t11153 = t11152 * t905;
    let t11155 = 0.23392894490538584828e1_f64 * t912 * t11153;
    let t11156 = t8749 * t1448;
    let t11157 = t8752 * t2595;
    (t11146, t11149, t11155, t11156, t11157)
}
