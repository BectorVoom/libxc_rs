//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1089/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1089(t577: f64, t703: f64, t7933: f64, t7934: f64, t76547: f64, t70618: f64, t76550: f64, t14530: f64, t534: f64, t72: f64, t72170: f64, t72178: f64, t72192: f64, t72193: f64, t76545: f64, t78591: f64, t78592: f64, t78593: f64, t78595: f64, t78597: f64, t78602: f64, t78605: f64) -> f64 {
    let t78608 = t7933 * t7934 * t577 * t703;
    let t78609 = 0.36021158228745895953e-3_f64 * t78608;
    let t78611 = 0.20496175532535769483e-3_f64 * t76547;
    let t78612 = 0.16263363996404810741e-4_f64 * t70618;
    let t78613 = 0.14967802127329760705e-1_f64 * t76550;
    let t78614 = t78591 + t78592 - t72170 - t78593 + t72178 + t78595 - t78597 + t72 * t534 * t14530 - t78602 - t78605 + t78609 - 0.40992351065071538964e-4_f64 * t76545 - t78611 - t72192 + t72193 + t78612 + t78613;
    t78614
}
