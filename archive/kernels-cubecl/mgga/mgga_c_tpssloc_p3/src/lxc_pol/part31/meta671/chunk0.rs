//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2001/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2001<F: Float>(t26016: F, t92047: F, t2031: F, t96425: F, t23967: F, t27972: F, t27976: F, t2032: F, t23963: F, t23970: F, t26009: F, t26954: F, t83717: F, t90098: F, t90114: F, t91954: F, t92057: F, t96422: F, t96443: F, t96473: F, t96535: F) -> F {
    let t102173 = t26016 * t92047;
    let t102187 = t2031 * t96425;
    let t102192 = t23967 * t27972;
    let t102194 = t23967 * t27976;
    let t102198 = -F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t102173 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t96473 * t23970 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t26016 * t92057 + F::cast_from(20.0_f64) * t91954 * t26009 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t90114 * t26954 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t96443 * t23970 + F::cast_from(20.0_f64) * t23963 * t96422 - F::cast_from(20.0_f64) * t83717 * t102187 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t90098 * t26954 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t102192 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t102194 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t96535 * t2032;
    t102198
}
