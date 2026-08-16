//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2524/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2524<F: Float>(t51483: F, t10069: F, t14588: F, t10518: F, t14606: F, t10073: F, t14504: F, t14575: F, t2435: F, t14568: F, t1568: F, t4503: F) -> (F, F, F, F, F, F, F) {
    let t51484 = F::cast_from(0.34697458558045176417e-2_f64) * t51483;
    let t51507 = t10069 * t14588;
    let t51512 = t14606 * t10518;
    let t51513 = F::cast_from(0.39029762157531132076e-1_f64) * t51512;
    let t51521 = t10073 * t14504;
    let t51522 = F::cast_from(0.19514881078765566038e-2_f64) * t51521;
    let t51537 = t2435 * t14575;
    let t51538 = F::cast_from(0.21951497276451705329e-1_f64) * t51537;
    let t51546 = t14568 * t10518;
    let t51547 = F::cast_from(0.39029762157531132076e-1_f64) * t51546;
    let t51548 = t4503 * t1568;
    (t51484, t51507, t51513, t51522, t51538, t51547, t51548)
}
