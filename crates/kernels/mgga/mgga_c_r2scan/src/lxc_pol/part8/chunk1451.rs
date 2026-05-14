//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1451/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1451<F: Float>(t1010: F, t23495: F, t23498: F, t2381: F, t2391: F, t2928: F, t2938: F, t31689: F, t31764: F, t321: F, t34951: F, t34994: F, t34997: F, t35070: F, t826: F, t8355: F, t8358: F, t9640: F, t9650: F, t9653: F, t9657: F) -> (F,) {
    let t35071 = (t34951 + t34994) * t321 - t34997 * t826 - 3.0 * t31764 * t1010 + 6.0 * t31689 * t2381 - 3.0 * t9640 * t2391 + 6.0 * t23495 * t2928 - 18.0 * t23498 * t9650 + 12.0 * t8358 * t9653 - 3.0 * t8355 * t2938 + 6.0 * t8358 * t9657 + t35070;
    (t35071,)
}
