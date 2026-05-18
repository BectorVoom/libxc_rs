//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1364/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1364<F: Float>(t1444: F, t1979: F, t1517: F, t2642: F, t1153: F, t12381: F, t12390: F, t12392: F, t1478: F, t1483: F, t1507: F, t16534: F, t16538: F, t17540: F, t17543: F, t17548: F, t17552: F, t1995: F, t2018: F, t368: F, t3810: F, t3842: F, t4193: F, t5527: F, t562: F, t5938: F, t86: F) -> F {
    let t17556 = t1979 * t1444;
    let t17558 = t1517 * t17556 * t2642;
    let t17562 = -F::new(0.619125e-2) * t562 * t16534 + F::new(0.9286875e-2) * t562 * t16538 - F::new(0.619125e-2) * t4193 * t1995 - F::new(0.123825e-1) * t1507 * t5527 + F::new(0.1857375e-1) * t5938 * t1478 - F::new(0.123825e-1) * t5938 * t1483 + F::new(0.9286875e-2) * t2018 * t3810 - F::new(0.619125e-2) * t2018 * t3842 + F::new(0.35374814814814814814e-1) * t12381 - F::new(0.35374814814814814814e-1) * t17540 - F::new(0.26531111111111111111e-1) * t1153 * t17543 - F::new(0.53062222222222222222e-1) * t1153 * t17548 - F::new(0.26531111111111111111e-1) * t12390 - F::new(0.39796666666666666666e-1) * t86 * t368 * t17552 + F::new(0.53062222222222222222e-1) * t1153 * t17558 - F::new(0.17687407407407407407e-1) * t12392;
    t17562
}
