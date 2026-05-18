//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1331/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1331<F: Float>(t167: F, t17102: F, t11952: F, t11954: F, t11958: F, t11960: F, t11962: F, t11967: F, t11974: F, t11977: F, t11985: F, t11987: F, t11995: F, t12003: F, t12005: F, t12009: F, t17096: F, t17098: F, t17100: F) -> F {
    let t17103 = t17102 * t167;
    let t17118 = F::new(0.23911438650126355246e-1) * t17096 - F::new(0.31077233446777841256e-3) * t17098 + F::new(0.11955719325063177623e0) * t17100 - F::new(0.72513544709148296264e-3) * t17103 - F::new(0.62154466893555682512e-3) * t11952 + F::new(0.10359077815592613752e-3) * t11954 + F::new(0.23911438650126355246e-1) * t11958 + F::new(0.47822877300252710492e-1) * t11960 - F::new(0.11955719325063177623e-1) * t11962 + t11967 - F::new(0.117630625e-4) * t11974 + F::new(0.15684083333333333333e-4) * t11977 + F::new(0.4684e-2) * t11985 - F::new(0.15613333333333333333e-2) * t11987 - F::new(0.9368e-2) * t11995 - F::new(0.21858666666666666666e-1) * t12003 + F::new(0.70444444444444444443e-2) * t12005 + F::new(0.78420416666666666666e-4) * t12009;
    t17118
}
