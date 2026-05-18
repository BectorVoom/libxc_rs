//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1253/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1253<F: Float>(t10829: F, t1976: F, t2874: F, t730: F, t10833: F, t17474: F, t17478: F, t721: F, t2849: F, t3625: F, t10963: F, t723: F) -> (F, F, F, F) {
    let t30731 = t1976 * t10829;
    let t30734 = F::new(0.17315859105681463759e2) * t730 * t30731 * t2874;
    let t30739 = F::new(0.91082604192152556044e5) * t730 * t17474 * t10833 * t17478 * t721;
    let t30742 = F::new(0.10526802520742363173e2) * t730 * t3625 * t2849;
    let t30745 = F::new(0.14035736694323150897e2) * t730 * t10963 * t723;
    (t30734, t30739, t30742, t30745)
}
