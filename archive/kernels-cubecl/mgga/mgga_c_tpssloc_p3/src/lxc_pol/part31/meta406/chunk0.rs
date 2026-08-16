//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1495/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1495<F: Float>(t25: F, t3701: F, t6463: F, t15909: F, t5127: F, t5187: F, t11987: F, t6305: F, t3704: F, t5397: F, t1298: F, t16557: F, t2219: F, t5170: F, t606: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t19596 = t6463 * t3701;
    let t19599 = F::cast_from(0.21687162600603479684e-1_f64) * t15909;
    let t19603 = t5127 * t5187;
    let t19606 = t11987 * t6305;
    let t19611 = t3704 * t5397;
    let t19617 = piecewise3::<F>(t26, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t19606 * t606 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t5170 * t2219 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19611 * t606 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1298 * t16557);
    (t19596, t19599, t19603, t19617)
}
