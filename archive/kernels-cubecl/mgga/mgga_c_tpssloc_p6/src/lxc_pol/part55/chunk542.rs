//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 542/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk542<F: Float>(t40: F, t182: F, t4095: F, t145: F, t4094: F, t185: F, t1472: F, t751: F, t1409: F, t707: F, t75: F, t3966: F, t607: F, t767: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t4097 = F::cast_from(0.19751673498613801407e-1_f64) * t4095 * t182;
    let t4098 = t145 * t4094;
    let t4099 = t4098 * t185;
    let t4100 = t1472 * t751;
    let t4101 = t751 * t1409;
    let t4102 = t707 * t4101;
    let t4103 = F::cast_from(4.0_f64) * t4102;
    let t4104 = t75 * t1409;
    let t4110 = piecewise3::<F>(t146, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4104 * t607 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t767 * t3966);
    (t4097, t4099, t4100, t4103, t4110)
}
