//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1241/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1241<F: Float>(t40821: F, t40840: F, t40844: F, t37029: F, t37041: F, t37063: F, t37066: F, t38792: F, t38808: F, t40817: F, t40825: F, t40828: F, t40830: F, t40833: F, t40835: F, t40837: F, t40842: F, t40848: F) -> F {
    let t41877 = F::new(8.0) * t40821;
    let t41885 = F::new(4.0) / F::new(3.0) * t40840;
    let t41887 = F::new(4.0) / F::new(3.0) * t40844;
    let t41890 = F::new(6.0) * t40817 + t41877 - F::new(3.0) * t40825 - F::new(3.0) / F::new(2.0) * t40828 + t40830 / F::new(4.0) + F::new(4.0) / F::new(3.0) * t37029 - t40833 - t40835 / F::new(2.0) - t40837 / F::new(4.0) + F::new(44.0) / F::new(9.0) * t37041 + t38792 - t41885 + F::new(3.0) / F::new(2.0) * t40842 + t41887 - F::new(44.0) / F::new(9.0) * t37066 + t38808 + F::new(2.0) / F::new(3.0) * t37063 + t40848;
    t41890
}
