//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2652/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2652<F: Float>(t40690: F, t5610: F, t13783: F, t13784: F, t13789: F, t13804: F, t36776: F, t3934: F, t3938: F, t46432: F, t46861: F, t46863: F, t46865: F, t48073: F, t48105: F, t48113: F, t48786: F, t48790: F, t48792: F, t48794: F, t48797: F, t48798: F, t48811: F, t48814: F, t48825: F, t48827: F, t5671: F, t9835: F, t9956: F) -> F {
    let t48829 = t40690 * t5610;
    let t48832 = F::cast_from(0.40656002247428262581e-3_f64) * t46861 + F::cast_from(0.10003937560882938627e-2_f64) * t46863 + F::cast_from(0.21437009059034868486e-4_f64) * t48786 + F::cast_from(0.17149607247227894789e-3_f64) * t48790 - F::cast_from(0.12846167376791569079e-2_f64) * t48792 + F::cast_from(0.34013387707001991331e0_f64) * t48794 - t48797 + F::cast_from(0.77173232612525526549e-1_f64) * t3934 * t48798 * t13784 * t9956 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t48113 * t3938 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t13789 * t48073 * t3938 - F::cast_from(0.12004725073059526352e-1_f64) * t48811 - t48814 - F::cast_from(0.1543464652250510531e-1_f64) * t5671 * t13789 * t48073 * t9835 - F::cast_from(0.38586616306262763276e-2_f64) * t13804 * t36776 * t48105 * t46432 - F::cast_from(0.15246000842785598468e-2_f64) * t48825 + F::cast_from(0.45732285992607719437e-2_f64) * t48827 + F::cast_from(0.11294745624363664198e-6_f64) * t48829 + F::new(7.0) / F::new(144.0) * t46865;
    t48832
}
