//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 766/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk766<F: Float>(t2064: F, t833: F, t1550: F, t1338: F, t2039: F, t357: F, t638: F, t132: F, t4781: F, t1343: F, t2040: F, t71: F, t830: F) -> (F, F, F, F, F) {
    let t35765 = t2064 * t833;
    let t35766 = t1550 * t35765;
    let t35772 = t638 * t2039 * t357 * t1338;
    let t35776 = t638 * t2039 * t132 * t4781;
    let t35777 = F::cast_from(0.15243824895787514157e-3_f64) * t35776;
    let t35781 = t638 * t830 * t1343 * t71 * t2040;
    (t35765, t35766, t35772, t35777, t35781)
}
