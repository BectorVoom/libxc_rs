//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 732/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk732<F: Float>(t1648: F, t4652: F, t1646: F, t4681: F, t10568: F, t10570: F, t10572: F, t10574: F, t10576: F, t10579: F, t10582: F, t10587: F, t10590: F, t10595: F, t10598: F, t10667: F, t10672: F, t10675: F) -> (F, F, F) {
    let t11358 = t1648 * t4652;
    let t11361 = t1646 * t4681;
    let t11371 = F::cast_from(0.12841111111111111111e-1_f64) * t10568;
    let t11382 = F::cast_from(0.14865e-1_f64) * t10672 - F::cast_from(0.2973e-1_f64) * t10675 + F::cast_from(0.1982e-1_f64) * t10667 - t11371 - F::cast_from(0.55033333333333333332e-2_f64) * t10570 + F::cast_from(0.27516666666666666666e-2_f64) * t10572 - F::cast_from(0.82549999999999999999e-2_f64) * t10574 + F::cast_from(0.41274999999999999999e-2_f64) * t10576 - F::cast_from(0.45861111111111111112e-2_f64) * t10579 + F::cast_from(0.1651e-1_f64) * t10582 - F::cast_from(0.82550000000000000001e-2_f64) * t10587 - F::cast_from(0.24765e-1_f64) * t10590 + F::cast_from(0.24765e-1_f64) * t10595 - F::cast_from(0.41275e-2_f64) * t10598;
    (t11358, t11361, t11382)
}
