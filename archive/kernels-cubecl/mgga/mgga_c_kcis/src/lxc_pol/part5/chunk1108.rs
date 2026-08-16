//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1108/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1108<F: Float>(t1109: F, t18685: F, t345: F, t10414: F, t1102: F, t14125: F, t14127: F, t14137: F, t14168: F, t14202: F, t14204: F, t14321: F, t1697: F, t18597: F, t18601: F, t18606: F, t18608: F, t18615: F, t18620: F, t18624: F, t18627: F, t18632: F, t18636: F, t18639: F, t344: F, t4768: F, t6432: F, t975: F) -> (F, F) {
    let t18686 = t1109 * t18685;
    let t18687 = t345 * t18686;
    let t18690 = -F::cast_from(0.1478346675e-2_f64) * t1102 * t18597 + F::cast_from(0.19711289e-2_f64) * t1102 * t18601 + t14125 - t14127 - F::cast_from(4.0_f64) * t975 * t6432 + F::cast_from(0.98556445e-3_f64) * t18606 + F::cast_from(0.13140859333333333333e-2_f64) * t18608 - F::cast_from(0.87605728888888888887e-3_f64) * t14137 + t14168 - F::cast_from(8.0_f64) * t1697 * t4768 - t14202 + t14204 - F::cast_from(0.19711289e-2_f64) * t10414 * t18615 + F::cast_from(0.26281718666666666666e-2_f64) * t10414 * t18620 + F::cast_from(0.26281718666666666666e-2_f64) * t10414 * t18624 - F::cast_from(0.21901432222222222222e-2_f64) * t14321 * t18627 + F::cast_from(0.98556445e-3_f64) * t10414 * t18632 - F::cast_from(0.19711289e-2_f64) * t10414 * t18636 - F::cast_from(0.39422578e-2_f64) * t10414 * t18639 + F::cast_from(0.1478346675e-2_f64) * t344 * t18687;
    (t18686, t18690)
}
