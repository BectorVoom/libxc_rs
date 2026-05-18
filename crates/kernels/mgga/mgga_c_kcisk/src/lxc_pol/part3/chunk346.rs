//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 346/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk346<F: Float>(t1248: F, t1636: F, t1720: F, t1699: F, t1710: F, t1712: F, t1715: F, t1719: F, t620: F) -> (F, F, F) {
    let t1722 = t1248 * t1720 * t1636;
    let t1724 = F::new(0.1898925e1) * t1710 - t1712 - F::new(0.29896666666666666667e0) * t1699 + F::new(0.3071625e0) * t1715 - t1719 - F::new(0.16431333333333333333e0) * t1722;
    let t1725 = F::new(1.0) / t620;
    (t1722, t1724, t1725)
}
