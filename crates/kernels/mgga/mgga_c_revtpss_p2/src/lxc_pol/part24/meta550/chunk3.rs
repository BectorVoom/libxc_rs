//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1629/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1629<F: Float>(t14961: F, t1559: F, t23172: F, t40314: F, t40316: F, t4514: F, t51553: F, t62843: F, t62847: F, t62874: F, t62907: F, t76127: F, t77191: F, t77197: F, t820: F) -> F {
    let t87850 = -F::cast_from(0.26341796731742046395e1_f64) * t4514 * t76127 * t1559 + F::cast_from(0.13170898365871023197e0_f64) * t77191 + F::cast_from(0.21951497276451705328e-1_f64) * t77197 + F::cast_from(0.43902994552903410657e-1_f64) * t62843 - t40314 + t40316 - F::cast_from(0.39029762157531132076e-2_f64) * t62847 - F::cast_from(0.13878983423218070567e-1_f64) * t62874 + F::cast_from(0.15805078039045227836e2_f64) * t820 * t14961 * t23172 - F::cast_from(0.1040793657534163522e-1_f64) * t51553 + F::cast_from(0.13878983423218070567e-1_f64) * t62907;
    t87850
}
