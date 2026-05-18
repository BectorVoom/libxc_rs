//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 984/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk984<F: Float>(t50: F, t462: F, t951: F, t352: F, t39: F, t954: F, t1792: F, t343: F, t1789: F, t2966: F, t2967: F, t2973: F, t34: F, t4367: F, t4370: F, t52: F, t743: F, t8334: F, t9456: F, t950: F, zeta_threshold: F) -> (F, F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t11437 = t462 * t951;
    let t11445 = t39 * t352;
    let t11448 = t462 * t954;
    let t11456 = F::new(32.0) * t1792 * t343;
    let t11458 = piecewise3::<f64>(t51, F::new(0.0), F::new(40.0) / F::new(81.0) * t8334 * t743 * t2967 + F::new(16.0) / F::new(9.0) * t2966 * t34 * t11437 - F::new(8.0) / F::new(9.0) * t4367 * t9456 - F::new(8.0) / F::new(3.0) * t950 * t462 * t352 + F::new(8.0) * t4370 * t11445 - F::new(8.0) / F::new(3.0) * t4370 * t11448 + F::new(4.0) / F::new(9.0) * t1789 * t2973 + F::new(16.0) * t52 * t39 - t11456);
    (t11437, t11445, t11448, t11458)
}
