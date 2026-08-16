//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1097/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1097<F: Float>(t40: F, t52: F, t4101: F, t707: F, t1409: F, t75: F, t3966: F, t607: F, t767: F, t78: F, t771: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t4102 = t707 * t4101;
    let t4103 = F::cast_from(4.0_f64) * t4102;
    let t4104 = t75 * t1409;
    let t4110 = piecewise3::<F>(t146, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4104 * t607 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t767 * t3966);
    let t4111 = t78 * t1409;
    let t4117 = piecewise3::<F>(t150, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4111 * t607 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t771 * t3966);
    let t4119 = t4110 / F::cast_from(2.0_f64) + t4117 / F::cast_from(2.0_f64);
    (t4102, t4103, t4104, t4111, t4119)
}
