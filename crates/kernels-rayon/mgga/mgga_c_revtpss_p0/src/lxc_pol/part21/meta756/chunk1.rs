//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2652/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2652(t40690: f64, t5610: f64, t13783: f64, t13784: f64, t13789: f64, t13804: f64, t36776: f64, t3934: f64, t3938: f64, t46432: f64, t46861: f64, t46863: f64, t46865: f64, t48073: f64, t48105: f64, t48113: f64, t48786: f64, t48790: f64, t48792: f64, t48794: f64, t48797: f64, t48798: f64, t48811: f64, t48814: f64, t48825: f64, t48827: f64, t5671: f64, t9835: f64, t9956: f64) -> f64 {
    let t48829 = t40690 * t5610;
    let t48832 = 0.40656002247428262581e-3_f64 * t46861 + 0.10003937560882938627e-2_f64 * t46863 + 0.21437009059034868486e-4_f64 * t48786 + 0.17149607247227894789e-3_f64 * t48790 - 0.12846167376791569079e-2_f64 * t48792 + 0.34013387707001991331e0_f64 * t48794 - t48797 + 0.77173232612525526549e-1_f64 * t3934 * t48798 * t13784 * t9956 - 0.12862205435420921092e-1_f64 * t3934 * t13783 * t48113 * t3938 + 0.25724410870841842183e-2_f64 * t3934 * t13789 * t48073 * t3938 - 0.12004725073059526352e-1_f64 * t48811 - t48814 - 0.1543464652250510531e-1_f64 * t5671 * t13789 * t48073 * t9835 - 0.38586616306262763276e-2_f64 * t13804 * t36776 * t48105 * t46432 - 0.15246000842785598468e-2_f64 * t48825 + 0.45732285992607719437e-2_f64 * t48827 + 0.11294745624363664198e-6_f64 * t48829 + 7.0_f64 / 144.0_f64 * t46865;
    t48832
}
