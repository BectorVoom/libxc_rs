//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2534/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2534(t50948: f64, t50946: f64, t50950: f64, t50952: f64, t50954: f64, t50957: f64, t50961: f64, t50966: f64, t50968: f64, t50970: f64, t50972: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43816: f64, t44275: f64, t50976: f64, t50978: f64, t50987: f64, t50990: f64, t50994: f64) -> (f64, f64) {
    let t51310 = 0.13772666666666666666e1_f64 * t50948;
    let t51320 = 0.123954e2_f64 * t50946 + t51310 + 0.68863333333333333333e0_f64 * t50950 + 0.34431666666666666666e0_f64 * t50952 + 0.20659e1_f64 * t50954 - 0.103295e1_f64 * t50957 - 0.103295e1_f64 * t50961 - 0.61977000000000000001e1_f64 * t50966 + 0.13892666666666666667e0_f64 * t50968 + 0.69463333333333333334e-1_f64 * t50970 + 0.41678000000000000001e0_f64 * t50972;
    let t51332 = t44275 - 0.10805407407407407407e0_f64 * t50976 - 0.92617777777777777778e-1_f64 * t50978 + 0.68863333333333333332e0_f64 * t43780 + 0.13772666666666666666e1_f64 * t43782 + 0.68863333333333333332e0_f64 * t43784 - 0.103295e1_f64 * t43786 - 0.17215833333333333333e0_f64 * t43788 - 0.16068111111111111111e1_f64 * t43816 + 0.13892666666666666667e0_f64 * t50987 + 0.55570666666666666666e0_f64 * t50990 - 0.61977e1_f64 * t50994;
    (t51320, t51332)
}
