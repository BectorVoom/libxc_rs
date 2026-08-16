//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 995/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk995(t12000: f64, t14271: f64, t14299: f64, t14302: f64, t14303: f64, t1445: f64, t1562: f64, t1641: f64, t2854: f64, t3689: f64, t41705: f64, t41711: f64, t4614: f64, t46167: f64, t46168: f64, t46169: f64, t46170: f64, t46174: f64, t46175: f64, t46176: f64, t47850: f64, t47866: f64, t47869: f64, t47871: f64, t4820: f64, t4953: f64, t49922: f64, t574: f64, t597: f64, t6824: f64, t8097: f64) -> f64 {
    let t50583 = -0.23005755572352449806e1_f64 * t47850 - 0.15889106645266856298e0_f64 * t6824 * t4820 * t49922 + t46167 - t46168 - t46169 + t46170 - t46174 - t46175 + t46176 + 0.30674340763136599741e2_f64 * t597 * t4614 * t14271 - 0.13803453343411469884e2_f64 * t4953 * t14299 - 0.13803453343411469884e2_f64 * t1562 * t1445 * t8097 * t3689 - 0.13803453343411469884e2_f64 * t1562 * t1445 * t2854 * t12000 - 0.92023022289409799224e1_f64 * t1641 * t14303 - 0.12269736305254639897e2_f64 * t574 * t4614 * t14302 - 0.11916829983950142223e0_f64 * t47866 - 0.63904876589867916127e-1_f64 * t41705 - 0.63904876589867916127e-1_f64 * t41711 + 0.38342925953920749677e1_f64 * t47869 - t47871;
    t50583
}
