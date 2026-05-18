//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1238/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1238<F: Float>(t40844: F, t37066: F, t23495: F, t3363: F, t37029: F, t37039: F, t37063: F, t37076: F, t40817: F, t40822: F, t40825: F, t40828: F, t40830: F, t40833: F, t40835: F, t40837: F, t40839: F, t40841: F, t40842: F) -> F {
    let t40845 = F::new(2.0) / F::new(3.0) * t40844;
    let t40846 = F::new(22.0) / F::new(9.0) * t37066;
    let t40848 = t23495 * t3363;
    let t40850 = F::new(3.0) * t40817 + t40822 - F::new(3.0) / F::new(2.0) * t40825 - F::new(3.0) / F::new(4.0) * t40828 + t40830 / F::new(8.0) + F::new(2.0) / F::new(3.0) * t37029 - t40833 / F::new(2.0) - t40835 / F::new(4.0) - t40837 / F::new(8.0) + t40839 + t37039 - t40841 + F::new(3.0) / F::new(4.0) * t40842 + t40845 - t40846 + t37076 + t37063 / F::new(3.0) + t40848 / F::new(2.0);
    t40850
}
