//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1268/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1268<F: Float>(t23828: F, t2788: F, t4962: F, t19405: F, t19464: F, t19466: F, t23810: F, t23813: F, t23816: F, t23819: F, t23820: F, t23823: F, t23824: F, t881: F, t1509: F, t2483: F, t41: F) -> (F, F, F, F) {
    let t23829 = 0.32530743900905219526e-1 * t23828;
    let t23830 = t2788 * t4962;
    let t23831 = 0.32530743900905219526e-1 * t23830;
    let t23832 = -t19405 + t23810 - t23813 - t23816 - t23819 + t23820 - t23823 - 0.7089e1 * t881 * t23824 + 3.0 * t19464 + t19466 - t23829 + t23831;
    let t23834 = t41 * t2483 * t1509;
    (t23829, t23831, t23832, t23834)
}
