//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 838/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk838<F: Float>(t339: F, t8438: F, t341: F, t1028: F, t1030: F, t1310: F, t2426: F, t2430: F, t343: F, t839: F, t8463: F, t8465: F, t8467: F, t8469: F, t8471: F, t1339: F, t352: F) -> (F, F) {
    let t8473 = t339 * t8438;
    let t8475 = t341 * t8438;
    let t8479 = 0.3101306810232e2 * t2426 * t839 + 0.1550653405116e2 * t1028 * t1310 - 0.4355305902528e1 * t2430 * t839 - 0.2177652951264e1 * t1030 * t1310 - 0.8704e0 * t8463 - 0.17408e1 * t8465 - 0.8704e0 * t8467 - 0.4607056813647e1 * t8469 + 0.122462410087e2 * t8471 - 0.957855118103e1 * t8473 + 0.3101306810232e1 * t8475 - 0.362942158544e0 * t343 * t8438;
    let t8481 = t352 * t1339;
    (t8479, t8481)
}
