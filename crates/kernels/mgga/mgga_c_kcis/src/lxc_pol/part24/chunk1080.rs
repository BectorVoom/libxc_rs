//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1080/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1080<F: Float>(t27023: F, t28190: F, t27006: F, t96339: F, t96345: F, t26960: F, t96975: F, t27070: F, t28093: F, t96395: F, t96401: F, t96427: F, t1281: F, t28250: F, t4527: F, t7671: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t97387 = 0.23168402777777777778e-3 * t28190 * t27023;
    let t97407 = 0.7722800925925925926e-4 * t28190 * t27006;
    let t97420 = 0.10317654320987654321e-2 * t96339;
    let t97422 = 0.30952962962962962962e-2 * t96345;
    let t97428 = 0.7722800925925925926e-4 * t26960 * t96975;
    let t97431 = 0.30918233506944444444e-4 * t27070 * t28093;
    let t97442 = 0.10317654320987654321e-2 * t96395;
    let t97449 = 0.15476481481481481481e-2 * t96401;
    let t97465 = 0.23214722222222222222e-2 * t96427;
    let t97494 = t28250 * t1281;
    let t97561 = 2.0 * t4527 * t7671;
    (t97387, t97407, t97420, t97422, t97428, t97431, t97442, t97449, t97465, t97494, t97561)
}
