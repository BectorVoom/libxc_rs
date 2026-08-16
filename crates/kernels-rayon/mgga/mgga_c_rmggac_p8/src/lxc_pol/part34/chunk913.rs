//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 913/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk913(t5148: f64, t76077: f64, t69453: f64, t5259: f64, t74959: f64, t4669: f64, t74963: f64, t74815: f64, t556: f64, t69144: f64, t2842: f64, t69436: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76355 = 0.5987120850931904282e-1_f64 * t5148 * t76077;
    let t76356 = 0.79828278012425390427e-1_f64 * t69453;
    let t76358 = 0.5987120850931904282e-1_f64 * t5259 * t74959;
    let t76360 = 0.8980681276397856423e-1_f64 * t4669 * t74963;
    let t76362 = 0.5987120850931904282e-1_f64 * t5148 * t74815;
    let t76363 = t69144 * t556;
    let t76365 = t69436 * t2842;
    (t76355, t76356, t76358, t76360, t76362, t76363, t76365)
}
