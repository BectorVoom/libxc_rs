//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1074/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1074(t2049: f64, t35688: f64, t39116: f64, t7760: f64, t1982: f64, t7428: f64, t8602: f64, t8608: f64, t1627: f64, t2124: f64, t36624: f64, t36976: f64, t36984: f64, t4041: f64, t42142: f64, t42145: f64, t42149: f64, t42152: f64, t42156: f64, t42159: f64, t42162: f64, t42167: f64, t42170: f64, t4999: f64, t530: f64, t668: f64, t72: f64, t8824: f64, t903: f64) -> f64 {
    let t42174 = t35688 * t2049 * t39116 * t7760;
    let t42177 = t8602 * t7428 * t1982;
    let t42178 = 0.19863479950205658386e-4_f64 * t42177;
    let t42180 = t8608 * t7428 * t1982;
    let t42181 = 0.19863479950205658386e-4_f64 * t42180;
    let t42186 = -0.11971293719990017331e-4_f64 * t42142 - t42145 + 0.15961724959986689774e-4_f64 * t42149 - t42152 + 0.35922725105591425692e0_f64 * t903 * t2124 * t1627 - 0.14967802127329760705e-1_f64 * t42156 + 0.21819729323396273384e0_f64 * t36976 - t36984 - 0.17961362552795712846e0_f64 * t42159 - 0.5987120850931904282e-1_f64 * t42162 - 0.2363e1_f64 * t530 * t36624 - t42167 - 0.72042316457491791906e-3_f64 * t42170 + 0.10248087766267884742e-3_f64 * t42174 - t42178 - t42181 + t72 * t4999 * t668 + 0.11974241701863808564e0_f64 * t4041 * t8824;
    t42186
}
