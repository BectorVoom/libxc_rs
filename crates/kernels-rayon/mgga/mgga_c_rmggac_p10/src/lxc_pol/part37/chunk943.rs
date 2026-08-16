//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 943/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk943(t77080: f64, t74536: f64, t74539: f64, t74549: f64, t14451: f64, t1627: f64, t26287: f64, t8377: f64, t30204: f64, t1632: f64, t1635: f64, t26283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77081 = 0.34093327067806677161e-2_f64 * t77080;
    let t77082 = 0.1276937996798935182e-4_f64 * t74536;
    let t77083 = 0.1276937996798935182e-4_f64 * t74539;
    let t77084 = 0.15961724959986689775e-4_f64 * t74549;
    let t77085 = t14451 * t1627;
    let t77086 = t26287 * t77085;
    let t77087 = 0.8980681276397856423e-1_f64 * t77086;
    let t77088 = t14451 * t8377;
    let t77089 = t30204 * t77088;
    let t77090 = 0.5987120850931904282e-1_f64 * t77089;
    let t77091 = t14451 * t1632;
    let t77092 = t26287 * t77091;
    let t77093 = 0.8980681276397856423e-1_f64 * t77092;
    let t77094 = t14451 * t1635;
    let t77095 = t26283 * t77094;
    (t77081, t77082, t77083, t77084, t77085, t77087, t77088, t77090, t77091, t77093, t77094, t77095)
}
