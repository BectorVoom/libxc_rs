//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 824/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk824<F: Float>(t12: F, t24: F, t5158: F, t1064: F, t1430: F, t207: F, t3510: F, t3512: F, t439: F, t8729: F, t1165: F, t333: F, t3725: F, t3727: F, t507: F, t8742: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t8795 = F::cast_from(0.17315859105681463759e2_f64) * t5158;
    let t8805 = piecewise3::<F>(t84, F::new(0.0), F::new(8.0) / F::new(27.0) * t3510 * t439 - F::new(8.0) / F::new(9.0) * t1064 * t1430 - F::new(2.0) / F::new(9.0) * t3512 * t439 + F::new(2.0) / F::new(3.0) * t207 * t8729);
    let t8815 = piecewise3::<F>(t90, F::new(0.0), F::new(8.0) / F::new(27.0) * t3725 * t507 + F::new(8.0) / F::new(9.0) * t1165 * t1430 - F::new(2.0) / F::new(9.0) * t3727 * t507 + F::new(2.0) / F::new(3.0) * t333 * t8742);
    (t8795, t8805, t8815)
}
