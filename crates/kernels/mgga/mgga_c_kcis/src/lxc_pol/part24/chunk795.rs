//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 795/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk795<F: Float>(t1102: F, t18694: F, t18699: F, t18703: F, t18707: F, t18710: F, t18713: F, t18716: F, t18720: F, t18724: F, t18728: F, t18732: F, t18736: F, t18741: F, t18746: F, t18750: F, t18754: F, t18759: F, t18763: F, t18766: F, t18769: F, t4587: F) -> (F,) {
    let t18772 = 0.98556445e-3 * t1102 * t18694 + 0.13140859333333333333e-2 * t1102 * t18699 + 0.16426074166666666666e-2 * t1102 * t18703 - 0.10950716111111111111e-2 * t1102 * t18707 - 0.65704296666666666666e-2 * t1102 * t18710 + 0.29201909629629629629e-2 * t1102 * t18713 + 0.43802864444444444444e-2 * t4587 * t18716 + 0.19711289e-2 * t1102 * t18720 + 0.39422578e-2 * t4587 * t18724 - 0.13140859333333333333e-2 * t1102 * t18728 - 0.26281718666666666666e-2 * t4587 * t18732 + 0.65704296666666666667e-3 * t1102 * t18736 - 0.36958666875e-3 * t1102 * t18741 - 0.7391733375e-3 * t1102 * t18746 + 0.1478346675e-2 * t1102 * t18750 - 0.295669335e-2 * t1102 * t18754 - 0.65704296666666666667e-3 * t1102 * t18759 - 0.65704296666666666667e-3 * t1102 * t18763 - 0.13140859333333333333e-2 * t1102 * t18766 + 0.10950716111111111111e-2 * t1102 * t18769;
    (t18772,)
}
