//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1731/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1731<F: Float>(t2735: F, t9792: F, t1413: F, t46826: F, t1376: F, t40769: F, t3989: F, t9986: F, t1353: F, t1410: F, t4012: F, t46787: F, t46789: F, t46793: F, t46797: F, t46800: F, t46804: F, t46810: F, t46812: F, t46817: F, t46820: F, t46824: F, t46828: F, t46831: F, t46833: F, t828: F, t9628: F) -> F {
    let t46835 = t2735 * t9792;
    let t46837 = t46835 * t1413 * t46826;
    let t46840 = F::cast_from(0.70398079132139197745e-2_f64) * t40769 * t1376;
    let t46846 = t3989 * t9986;
    let t46848 = -F::cast_from(0.32131292352189751911e-5_f64) * t46787 - F::cast_from(0.45732285992607719437e-3_f64) * t46789 + F::cast_from(0.28582678745379824648e-4_f64) * t46793 - F::cast_from(0.17149607247227894789e-3_f64) * t46797 + t46800 + F::cast_from(0.54214778996945588149e-4_f64) * t46804 + t46810 - F::cast_from(0.27107389498472794074e-4_f64) * t46812 - t46817 + t46820 - t46824 + F::cast_from(0.2168591159877823526e-3_f64) * t46828 - t46831 + F::cast_from(0.32528867398167352889e-3_f64) * t46833 - F::cast_from(0.12196800674228478774e-3_f64) * t46837 + t46840 + F::cast_from(0.17149607247227894789e-1_f64) * t1410 * t4012 * t828 * t9628 * t1353 - F::cast_from(0.24009450146119052704e0_f64) * t46846;
    t46848
}
