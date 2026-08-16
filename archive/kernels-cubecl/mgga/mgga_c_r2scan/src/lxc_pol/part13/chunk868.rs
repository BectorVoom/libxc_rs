//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 868/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk868<F: Float>(t4791: F, t4794: F, t4798: F, t4806: F, t4992: F, t5999: F, t6002: F, t6961: F, t6975: F, t7009: F, t7865: F, t2823: F, t6001: F) -> (F, F) {
    let t7869 = -t6961 + F::cast_from(0.285764e-1_f64) * t7865 - t4791 + t4794 + t4798 - t4806 + t6975 + t4992 - F::cast_from(0.1350520664e0_f64) * t5999 - F::cast_from(0.1350520664e0_f64) * t6002 - t7009;
    let t7870 = t2823 * t6001;
    (t7869, t7870)
}
