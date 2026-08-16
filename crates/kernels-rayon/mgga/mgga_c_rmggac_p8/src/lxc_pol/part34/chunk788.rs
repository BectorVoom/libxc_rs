//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 788/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk788(t14084: f64, t38839: f64, t38844: f64, t14091: f64, t27: f64, t8430: f64, t16069: f64, t69609: f64, t8435: f64, t16074: f64, t15411: f64, t68761: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t74193 = t14084 * t38839;
    let t74195 = t14084 * t38844;
    let t74197 = t14091 * t38839;
    let t74199 = t14091 * t38844;
    let t74201 = t27 * t8430;
    let t74203 = t69609 * t16069 * t74201;
    let t74205 = t27 * t8435;
    let t74207 = t69609 * t16074 * t74205;
    let t74209 = t68761 * t15411;
    (t74193, t74195, t74197, t74199, t74203, t74207, t74209)
}
