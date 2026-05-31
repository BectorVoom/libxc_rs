//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1113/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1113<F: Float>(t18672: F, t4565: F, t1102: F, t18694: F, t18699: F, t18703: F, t18707: F, t18710: F, t18713: F, t18716: F, t18720: F, t18724: F, t18728: F, t18732: F, t18736: F, t18741: F, t18746: F, t18750: F, t18754: F, t18759: F, t18763: F, t18766: F, t4587: F) -> F {
    let t18769 = t4565 * t18672;
    let t18772 = F::cast_from(0.98556445e-3_f64) * t1102 * t18694 + F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t18699 + F::cast_from(0.16426074166666666666e-2_f64) * t1102 * t18703 - F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t18707 - F::cast_from(0.65704296666666666666e-2_f64) * t1102 * t18710 + F::cast_from(0.29201909629629629629e-2_f64) * t1102 * t18713 + F::cast_from(0.43802864444444444444e-2_f64) * t4587 * t18716 + F::cast_from(0.19711289e-2_f64) * t1102 * t18720 + F::cast_from(0.39422578e-2_f64) * t4587 * t18724 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t18728 - F::cast_from(0.26281718666666666666e-2_f64) * t4587 * t18732 + F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t18736 - F::cast_from(0.36958666875e-3_f64) * t1102 * t18741 - F::cast_from(0.7391733375e-3_f64) * t1102 * t18746 + F::cast_from(0.1478346675e-2_f64) * t1102 * t18750 - F::cast_from(0.295669335e-2_f64) * t1102 * t18754 - F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t18759 - F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t18763 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t18766 + F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t18769;
    t18772
}
