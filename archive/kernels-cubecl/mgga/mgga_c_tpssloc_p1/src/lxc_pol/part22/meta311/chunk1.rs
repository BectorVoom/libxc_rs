//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1486/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1486<F: Float>(t11352: F, t1682: F, t14722: F, t14704: F, t1675: F, t3331: F, t3403: F, t4857: F, t11285: F, t1694: F, t15026: F, t3623: F) -> (F, F, F, F, F, F, F) {
    let t15171 = t1682 * t11352;
    let t15194 = F::cast_from(0.2283111111111111111e-1_f64) * t14722;
    let t15195 = F::cast_from(0.11415555555555555555e-1_f64) * t14704;
    let t15207 = t1675 * t3331;
    let t15218 = t4857 * t3403;
    let t15225 = t1694 * t11285;
    let t15245 = t15026 * t3623;
    (t15171, t15194, t15195, t15207, t15218, t15225, t15245)
}
