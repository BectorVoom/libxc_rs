//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 863/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk863<F: Float>(t10280: F, t38234: F, t38235: F, t38236: F, t38237: F, t38238: F, t38239: F, t7384: F, t9309: F, t9764: F, t9767: F, t9335: F, t9336: F, t9785: F, t9787: F, t9792: F, t9794: F, t9797: F, t9801: F, t9805: F, t9809: F, t9811: F) -> (F, F) {
    let t44533 = -t38234 - t38235 + t10280 + t38236 - t38237 + t9764 + t38238 + t9309 - t9767 + t7384 + t38239;
    let t44540 = -t9785 - t9787 - t9792 + t9794 + t9797 - t9801 - t9805 + t9809 - t9811 + t9335 + t9336;
    (t44533, t44540)
}
