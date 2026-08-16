//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 905/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk905(t25820: f64, t74977: f64, t14174: f64, t15093: f64, t14170: f64, t75411: f64, t4669: f64, t74805: f64, t15087: f64, t40826: f64, t5259: f64, t76048: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76199 = t25820 * t74977;
    let t76201 = t15093 * t14174;
    let t76203 = t75411 * t14170;
    let t76212 = 0.8980681276397856423e-1_f64 * t4669 * t74805;
    let t76216 = 0.5987120850931904282e-1_f64 * t40826 * t15087;
    let t76218 = 0.5987120850931904282e-1_f64 * t5259 * t76048;
    (t76199, t76201, t76203, t76212, t76216, t76218)
}
