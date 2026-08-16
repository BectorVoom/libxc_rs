//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1731/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1731(t2735: f64, t9792: f64, t1413: f64, t46826: f64, t1376: f64, t40769: f64, t3989: f64, t9986: f64, t1353: f64, t1410: f64, t4012: f64, t46787: f64, t46789: f64, t46793: f64, t46797: f64, t46800: f64, t46804: f64, t46810: f64, t46812: f64, t46817: f64, t46820: f64, t46824: f64, t46828: f64, t46831: f64, t46833: f64, t828: f64, t9628: f64) -> f64 {
    let t46835 = t2735 * t9792;
    let t46837 = t46835 * t1413 * t46826;
    let t46840 = 0.70398079132139197745e-2_f64 * t40769 * t1376;
    let t46846 = t3989 * t9986;
    let t46848 = -0.32131292352189751911e-5_f64 * t46787 - 0.45732285992607719437e-3_f64 * t46789 + 0.28582678745379824648e-4_f64 * t46793 - 0.17149607247227894789e-3_f64 * t46797 + t46800 + 0.54214778996945588149e-4_f64 * t46804 + t46810 - 0.27107389498472794074e-4_f64 * t46812 - t46817 + t46820 - t46824 + 0.2168591159877823526e-3_f64 * t46828 - t46831 + 0.32528867398167352889e-3_f64 * t46833 - 0.12196800674228478774e-3_f64 * t46837 + t46840 + 0.17149607247227894789e-1_f64 * t1410 * t4012 * t828 * t9628 * t1353 - 0.24009450146119052704e0_f64 * t46846;
    t46848
}
