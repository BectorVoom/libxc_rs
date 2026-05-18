//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1082/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1082<F: Float>(t1550: F, t42970: F, t44410: F, t45493: F, t45495: F, t45499: F, t45503: F, t45505: F, t45507: F, t45509: F, t45514: F, t45519: F, t45523: F, t45525: F, t45531: F, t45536: F, t45541: F, t530: F, t6412: F, t699: F) -> F {
    let t48520 = F::new(0.15323255961587222184e-3) * t45493 - F::new(0.4726e1) * t530 * t44410 + F::new(0.5107751987195740728e-4) * t45495 - F::new(0.212822999466489197e-4) * t45499 - F::new(0.5107751987195740728e-4) * t45503 + t42970 + F::new(0.19863479950205658386e-4) * t45505 - F::new(0.23948483403727617128e0) * t1550 * t699 * t6412 - F::new(0.212822999466489197e-4) * t45507 - F::new(0.212822999466489197e-4) * t45509 - F::new(0.212822999466489197e-4) * t45514 + F::new(0.1702583995731913576e-4) * t45519 - F::new(0.1064114997332445985e-4) * t45523 - F::new(0.5107751987195740728e-4) * t45525 + F::new(0.68186654135613354325e-2) * t45531 + F::new(0.212822999466489197e-4) * t45536 - F::new(0.212822999466489197e-4) * t45541;
    t48520
}
