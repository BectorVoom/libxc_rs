//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1285/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1285(t1394: f64, t7100: f64, t94216: f64, t22285: f64, t27387: f64, t22290: f64, t5780: f64, t101853: f64, t101994: f64, t101997: f64, t102001: f64, t102005: f64, t28708: f64, t28727: f64, t28844: f64, t28853: f64, t7978: f64, t7981: f64, t98162: f64) -> (f64, f64, f64, f64) {
    let t102011 = t1394 * t94216 * t7100;
    let t102014 = t1394 * t27387 * t22285;
    let t102017 = t5780 * t27387 * t22290;
    let t102025 = -0.51588271604938271603e-3_f64 * t98162 + 0.61905925925925925925e-2_f64 * t101994 + 0.20635308641975308642e-2_f64 * t101997 + 0.69644166666666666664e-2_f64 * t102001 - 0.69505208333333333334e-3_f64 * t7978 * t102005 + 0.61782407407407407407e-3_f64 * t101853 * t7981 - 0.23214722222222222222e-2_f64 * t102011 - 0.23214722222222222222e-2_f64 * t102014 + 0.46429444444444444444e-2_f64 * t102017 + 0.74203760416666666667e-3_f64 * t28853 * t28708 + 0.37069444444444444444e-2_f64 * t28727 * t28708 - 0.12356481481481481482e-2_f64 * t28727 * t28844;
    (t102011, t102014, t102017, t102025)
}
