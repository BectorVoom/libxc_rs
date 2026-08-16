//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1297/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1297<F: Float>(t18210: F, t29509: F, t7978: F, t1464: F, t1489: F, t27387: F, t7309: F, t102166: F, t102170: F, t102174: F, t102180: F, t102183: F, t1650: F, t27556: F, t27567: F, t27583: F, t27584: F, t28708: F, t29514: F, t29526: F, t4440: F, t6183: F, t98380: F, t99133: F) -> (F, F, F) {
    let t102190 = t18210 * t29509;
    let t102191 = t7978 * t102190;
    let t102197 = t1464 * t27387 * t7309 * t1489;
    let t102200 = F::cast_from(0.18550940104166666667e-3_f64) * t27567 * t102166 + F::cast_from(0.2782641015625e-3_f64) * t27567 * t102170 - F::cast_from(0.3861400462962962963e-4_f64) * t102174 + F::cast_from(0.49512459138020833334e-4_f64) * t99133 * t28708 - F::cast_from(0.92754700520833333334e-4_f64) * t27556 * t29526 - F::cast_from(0.15476481481481481481e-2_f64) * t102180 - F::cast_from(0.23214722222222222222e-2_f64) * t102183 + F::cast_from(0.23168402777777777778e-3_f64) * t27583 * t4440 * t27584 * t1650 * t6183 + F::cast_from(0.11584201388888888889e-3_f64) * t102191 - F::cast_from(0.13913205078125e-3_f64) * t27556 * t29514 + F::cast_from(0.38691203703703703703e-3_f64) * t102197 + F::cast_from(0.46429444444444444444e-2_f64) * t98380;
    (t102190, t102197, t102200)
}
