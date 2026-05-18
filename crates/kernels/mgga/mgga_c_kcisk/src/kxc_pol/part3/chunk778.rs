//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 778/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk778<F: Float>(t10501: F, t1992: F, t772: F, t5520: F, t9726: F, t1961: F, t5372: F, t10568: F, t10570: F, t10572: F, t10574: F, t10576: F, t10579: F, t10582: F, t10587: F, t10590: F, t10595: F, t10598: F) -> (F, F, F, F, F) {
    let t11983 = F::new(0.51588271604938271604e-3) * t10501;
    let t11984 = t1992 * t1992;
    let t11985 = F::new(1.0) / t11984;
    let t11986 = t772 * t11985;
    let t11991 = t9726 * t5520;
    let t11999 = t1961 * t5372;
    let t12002 = F::new(0.53272592592592592592e-1) * t10568;
    let t12013 = -t12002 - F::new(0.2283111111111111111e-1) * t10570 + F::new(0.11415555555555555555e-1) * t10572 - F::new(0.34246666666666666665e-1) * t10574 + F::new(0.17123333333333333333e-1) * t10576 - F::new(0.19025925925925925925e-1) * t10579 + F::new(0.68493333333333333331e-1) * t10582 - F::new(0.34246666666666666665e-1) * t10587 - F::new(0.10274e0) * t10590 + F::new(0.10274e0) * t10595 - F::new(0.17123333333333333333e-1) * t10598;
    (t11983, t11986, t11991, t11999, t12013)
}
