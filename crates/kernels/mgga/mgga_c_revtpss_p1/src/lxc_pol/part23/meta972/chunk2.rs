//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3293/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3293<F: Float>(t22953: F, t555: F, t22954: F, t4101: F, t686: F, t72: F, t1892: F, t6861: F, t14193: F, t1883: F, t21990: F, t22005: F, t22016: F, t46515: F, t46518: F, t48080: F, t48082: F, t48090: F, t5675: F, t5745: F, t5755: F, t74965: F, t75060: F) -> (F, F, F) {
    let t86455 = t555 * t22953;
    let t86468 = t4101 * t22954 * t72 * t686;
    let t86470 = t1892 * t6861;
    let t86474 = -t46515 + F::cast_from(0.13170898365871023197e1_f64) * t5745 * t86455 * t5675 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t74965 * t1883 + t48080 + t48082 + F::cast_from(0.11853808529283920877e2_f64) * t5745 * t22005 * t21990 + t48090 + F::cast_from(0.58544643236296698113e-1_f64) * t75060 - F::cast_from(0.9757440539382783019e-2_f64) * t86468 + t46518 - F::cast_from(0.11853808529283920877e2_f64) * t14193 * t86470 * t22016;
    (t86455, t86470, t86474)
}
