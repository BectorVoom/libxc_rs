//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1099/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1099<F: Float>(t10731: F, t10741: F, t829: F, t4148: F, t820: F, t10703: F, t848: F, t4175: F, t839: F, t1359: F, t1371: F, t2246: F, t2285: F, t3366: F, t3386: F, t4154: F, t4167: F, t4170: F, t4194: F, t4197: F, t6636: F, t6678: F, t6722: F, t821: F, t830: F, t840: F, t849: F, t8857: F, t8911: F) -> (F, F, F, F, F, F) {
    let t10742 = t10731 + t10741;
    let t10743 = t10742 * t829;
    let t10746 = t4148 * t820;
    let t10759 = t10703 * t848;
    let t10766 = t4175 * t839;
    let t10771 = F::new(1.0) * t821 * t10743 + F::new(1.0) * t10746 * t830 + F::new(2.0) * t8857 * t1359 + F::new(2.0) * t3366 * t3386 - F::new(2.0) * t6722 * t4154 + F::new(1.0) * t2246 * t4167 + F::cast_from(0.5848223622634646207e0_f64) * t2285 * t4194 + F::cast_from(0.5848223622634646207e0_f64) * t840 * t10759 + F::cast_from(0.17315859105681463759e2_f64) * t6636 * t4197 + F::cast_from(0.32163958997385070134e2_f64) * t6678 * t4170 + F::cast_from(0.5848223622634646207e0_f64) * t10766 * t849 + F::cast_from(0.11696447245269292414e1_f64) * t8911 * t1371;
    (t10742, t10743, t10746, t10759, t10766, t10771)
}
