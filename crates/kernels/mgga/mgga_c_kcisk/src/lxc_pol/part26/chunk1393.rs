//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1393/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1393<F: Float>(t33870: F, t9860: F, t3532: F, t4350: F, t1596: F, t7706: F, t33922: F, t120292: F, t32439: F, t109518: F, t109664: F, t115085: F, t115157: F, t120067: F, t25342: F, t33771: F, t33830: F, t33851: F, t33911: F, t33916: F, t33925: F, t33941: F, t34931: F, t6204: F, t83438: F, t9536: F, t9855: F) -> (F, F, F) {
    let t120621 = t9860 * t33870;
    let t120629 = t4350 * t3532;
    let t120630 = t7706 * t1596;
    let t120632 = t33922 * t120629 * t120630;
    let t120639 = t32439 * t120292;
    let t120647 = 0.69444444444444444444e-2 * t33941 * t33916 + 0.13402777777777777778e-2 * t115085 * t33911 - 0.46296296296296296296e-2 * t33941 * t33925 + 0.34722222222222222223e-2 * t33941 * t33771 + 0.34722222222222222223e-2 * t120621 - 0.27777777777777777779e-1 * t33851 * t9855 + 0.13888888888888888889e-1 * t9536 * t33922 * t115157 * t25342 + 0.89351851851851851851e-3 * t32439 * t120632 - 0.40208333333333333334e-2 * t109664 * t34931 - 0.40208333333333333334e-2 * t109518 * t34931 - 0.13402777777777777778e-2 * t120639 - 0.40208333333333333334e-2 * t32439 * t120067 - 0.20833333333333333334e-1 * t9536 * t6204 * t33830 * t83438;
    (t120630, t120632, t120647)
}
