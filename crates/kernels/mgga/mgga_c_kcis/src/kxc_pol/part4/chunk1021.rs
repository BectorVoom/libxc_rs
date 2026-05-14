//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1021/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1021<F: Float>(t13495: F, t4947: F, t1662: F, t2911: F, t9924: F, t13480: F, t4939: F, t2635: F, t4961: F, t2894: F, t1704: F, t2844: F, t2630: F, t9933: F, t14439: F, t14442: F, t14446: F, t14450: F, t14455: F, t14460: F, t1706: F, t2867: F, t2872: F, t4953: F, t4968: F, t991: F) -> (F,) {
    let t14463 = t4947 * t13495;
    let t14466 = t1662 * t2911;
    let t14467 = t9924 * t14466;
    let t14470 = t4939 * t13480;
    let t14473 = t4961 * t2635;
    let t14474 = t2894 * t14473;
    let t14477 = t1704 * t2844;
    let t14478 = t14477 * t2630;
    let t14479 = t9933 * t14478;
    let t14482 = -11.0 / 108.0 * t2867 * t1706 + t14439 - t14442 - t14446 + t14450 - t2872 * t4953 / 27.0 - 7.0 / 432.0 * t14455 - t2872 * t4968 / 9.0 + t991 * t14460 / 48.0 + t991 * t14463 / 48.0 + t991 * t14467 / 144.0 - t991 * t14470 / 36.0 - t991 * t14474 / 288.0 - t991 * t14479 / 216.0;
    (t14482,)
}
