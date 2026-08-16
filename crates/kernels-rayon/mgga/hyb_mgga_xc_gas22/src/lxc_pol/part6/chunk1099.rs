//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1099/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1099(t10731: f64, t10741: f64, t829: f64, t4148: f64, t820: f64, t10703: f64, t848: f64, t4175: f64, t839: f64, t1359: f64, t1371: f64, t2246: f64, t2285: f64, t3366: f64, t3386: f64, t4154: f64, t4167: f64, t4170: f64, t4194: f64, t4197: f64, t6636: f64, t6678: f64, t6722: f64, t821: f64, t830: f64, t840: f64, t849: f64, t8857: f64, t8911: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10742 = t10731 + t10741;
    let t10743 = t10742 * t829;
    let t10746 = t4148 * t820;
    let t10759 = t10703 * t848;
    let t10766 = t4175 * t839;
    let t10771 = 1.0_f64 * t821 * t10743 + 1.0_f64 * t10746 * t830 + 2.0_f64 * t8857 * t1359 + 2.0_f64 * t3366 * t3386 - 2.0_f64 * t6722 * t4154 + 1.0_f64 * t2246 * t4167 + 0.5848223622634646207e0_f64 * t2285 * t4194 + 0.5848223622634646207e0_f64 * t840 * t10759 + 0.17315859105681463759e2_f64 * t6636 * t4197 + 0.32163958997385070134e2_f64 * t6678 * t4170 + 0.5848223622634646207e0_f64 * t10766 * t849 + 0.11696447245269292414e1_f64 * t8911 * t1371;
    (t10742, t10743, t10746, t10759, t10766, t10771)
}
