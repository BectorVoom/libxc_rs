//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1067/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1067<F: Float>(t14656: F, t285: F, t4923: F, t8772: F, t3908: F, t912: F, t2593: F, t4939: F, t905: F, t10980: F, t11109: F, t11110: F, t11111: F, t14459: F, t14492: F, t14495: F, t14505: F, t14507: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t8616: F, t8756: F) -> (F, F, F, F) {
    let t14658 = F::cast_from(0.621814e-1_f64) * t14656 * t285;
    let t14659 = t8772 * t4923;
    let t14660 = t14659 * t3908;
    let t14662 = F::cast_from(0.10389515463408878255e3_f64) * t912 * t14660;
    let t14663 = t2593 * t4939;
    let t14664 = t14663 * t905;
    let t14666 = F::cast_from(0.11696447245269292414e1_f64) * t912 * t14664;
    let t14680 = -t8756 - F::cast_from(0.41203703703703703703e-2_f64) * t8616 - F::cast_from(0.82407407407407407408e-2_f64) * t10980 + t11109 - t11110 + t11111 + F::cast_from(0.20601851851851851852e-2_f64) * t14495 - F::cast_from(0.10300925925925925926e-1_f64) * t14517 + F::cast_from(0.37083333333333333333e-1_f64) * t14459 - F::cast_from(0.12361111111111111111e-1_f64) * t14521 - F::cast_from(0.61805555555555555557e-2_f64) * t14505 - F::cast_from(0.55625000000000000001e-1_f64) * t14525 + F::cast_from(0.37083333333333333334e-1_f64) * t14528 + F::cast_from(0.30902777777777777778e-2_f64) * t14507 - F::cast_from(0.61805555555555555555e-2_f64) * t14532 + F::cast_from(0.18541666666666666667e-1_f64) * t14535 - F::cast_from(0.92708333333333333333e-2_f64) * t14492;
    (t14658, t14662, t14666, t14680)
}
