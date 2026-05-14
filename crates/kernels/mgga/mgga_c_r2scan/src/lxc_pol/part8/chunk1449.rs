//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1449/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1449<F: Float>(t10443: F, t818: F, t1216: F, t1217: F, t2904: F, t2920: F, t34919: F, t34923: F, t34927: F, t34930: F, t34934: F, t34946: F, t34959: F, t34967: F, t806: F, t810: F, t8315: F, t8336: F, t8377: F, t8385: F, t9597: F, t9622: F) -> (F, F) {
    let t34997 = t10443 * t818;
    let t35065 = 40.0 / 81.0 * t34927 - 10.0 / 9.0 * t9597 * t1217 - 10.0 / 9.0 * t8315 * t2904 * t806 + 10.0 / 3.0 * t8377 * t1216 * t2904 + 10.0 / 3.0 * t34930 + 10.0 / 9.0 * t34934 + 5.0 / 3.0 * t34959 + 40.0 / 81.0 * t34946 + 10.0 / 9.0 * t9622 * t1217 - 10.0 / 9.0 * t8336 * t2920 * t810 - 10.0 / 3.0 * t8385 * t1216 * t2920 + 10.0 / 3.0 * t34919 + 10.0 / 9.0 * t34923 + 5.0 / 3.0 * t34967;
    (t34997, t35065)
}
