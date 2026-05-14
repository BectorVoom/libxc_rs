//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 657/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk657<F: Float>(t1102: F, t344: F, t3743: F, t3744: F, t3746: F, t3748: F, t3757: F, t3763: F, t3768: F, t3773: F, t3777: F, t3783: F, t3789: F, t3811: F, t3817: F, t3843: F, t3846: F, t3951: F, t486: F) -> (F,) {
    let t3954 = -t3743 + 0.8760572888888888889e-3 * t3744 + 0.19711289e-2 * t3746 - 0.13140859333333333333e-2 * t3748 + 0.10950716111111111111e-2 * t1102 * t3757 + 0.19711289e-2 * t1102 * t3763 - 0.13140859333333333333e-2 * t1102 * t3768 - 0.13140859333333333333e-2 * t1102 * t3773 + 0.65704296666666666667e-3 * t1102 * t3777 + 0.7391733375e-3 * t344 * t3783 - 0.295669335e-2 * t1102 * t3789 + 0.1478346675e-2 * t344 * t3811 + 0.19711289e-2 * t344 * t3817 - 0.98556445e-3 * t344 * t3843 - 4.0 * t3846 - 4.0 * t486 * t3951;
    (t3954,)
}
