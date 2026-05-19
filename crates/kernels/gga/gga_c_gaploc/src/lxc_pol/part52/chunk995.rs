//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 995/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk995<F: Float>(t12000: F, t14271: F, t14299: F, t14302: F, t14303: F, t1445: F, t1562: F, t1641: F, t2854: F, t3689: F, t41705: F, t41711: F, t4614: F, t46167: F, t46168: F, t46169: F, t46170: F, t46174: F, t46175: F, t46176: F, t47850: F, t47866: F, t47869: F, t47871: F, t4820: F, t4953: F, t49922: F, t574: F, t597: F, t6824: F, t8097: F) -> F {
    let t50583 = -F::cast_from(0.23005755572352449806e1_f64) * t47850 - F::cast_from(0.15889106645266856298e0_f64) * t6824 * t4820 * t49922 + t46167 - t46168 - t46169 + t46170 - t46174 - t46175 + t46176 + F::cast_from(0.30674340763136599741e2_f64) * t597 * t4614 * t14271 - F::cast_from(0.13803453343411469884e2_f64) * t4953 * t14299 - F::cast_from(0.13803453343411469884e2_f64) * t1562 * t1445 * t8097 * t3689 - F::cast_from(0.13803453343411469884e2_f64) * t1562 * t1445 * t2854 * t12000 - F::cast_from(0.92023022289409799224e1_f64) * t1641 * t14303 - F::cast_from(0.12269736305254639897e2_f64) * t574 * t4614 * t14302 - F::cast_from(0.11916829983950142223e0_f64) * t47866 - F::cast_from(0.63904876589867916127e-1_f64) * t41705 - F::cast_from(0.63904876589867916127e-1_f64) * t41711 + F::cast_from(0.38342925953920749677e1_f64) * t47869 - t47871;
    t50583
}
