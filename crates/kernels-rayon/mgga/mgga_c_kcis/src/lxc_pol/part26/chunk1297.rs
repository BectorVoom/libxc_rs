//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1297/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1297(t18210: f64, t29509: f64, t7978: f64, t1464: f64, t1489: f64, t27387: f64, t7309: f64, t102166: f64, t102170: f64, t102174: f64, t102180: f64, t102183: f64, t1650: f64, t27556: f64, t27567: f64, t27583: f64, t27584: f64, t28708: f64, t29514: f64, t29526: f64, t4440: f64, t6183: f64, t98380: f64, t99133: f64) -> (f64, f64, f64) {
    let t102190 = t18210 * t29509;
    let t102191 = t7978 * t102190;
    let t102197 = t1464 * t27387 * t7309 * t1489;
    let t102200 = 0.18550940104166666667e-3_f64 * t27567 * t102166 + 0.2782641015625e-3_f64 * t27567 * t102170 - 0.3861400462962962963e-4_f64 * t102174 + 0.49512459138020833334e-4_f64 * t99133 * t28708 - 0.92754700520833333334e-4_f64 * t27556 * t29526 - 0.15476481481481481481e-2_f64 * t102180 - 0.23214722222222222222e-2_f64 * t102183 + 0.23168402777777777778e-3_f64 * t27583 * t4440 * t27584 * t1650 * t6183 + 0.11584201388888888889e-3_f64 * t102191 - 0.13913205078125e-3_f64 * t27556 * t29514 + 0.38691203703703703703e-3_f64 * t102197 + 0.46429444444444444444e-2_f64 * t98380;
    (t102190, t102197, t102200)
}
