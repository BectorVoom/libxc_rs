//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1124/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1124<F: Float>(t28778: F, t7968: F, t27462: F, t27668: F, t28465: F, t28474: F, t28477: F, t28501: F, t28506: F, t28508: F, t28511: F, t28514: F, t28517: F, t28520: F, t28526: F, t28529: F, t28532: F, t28547: F, t28727: F, t28844: F, t28853: F, t7971: F, t7978: F) -> F {
    let t28856 = t7968 * t28778;
    let t28867 = F::new(0.11607361111111111111e-2) * t27462 + F::new(0.11607361111111111111e-2) * t28465 + F::new(0.23168402777777777778e-3) * t7978 * t28844 + F::new(0.17411041666666666666e-2) * t28474 - F::new(0.46429444444444444443e-2) * t28477 + F::new(0.11607361111111111111e-2) * t28501 + F::new(0.34822083333333333332e-2) * t28506 - F::new(0.92673611111111111112e-3) * t28727 * t7971 - F::new(0.12367293402777777778e-3) * t28853 * t7971 + F::new(0.15459116753472222222e-4) * t28856 - F::new(0.30952962962962962962e-2) * t28508 + F::new(0.77382407407407407407e-3) * t28511 - F::new(0.23214722222222222222e-2) * t28514 + F::new(0.19345601851851851852e-2) * t28517 - F::new(0.11607361111111111111e-2) * t28520 - t27668 - F::new(0.17411041666666666666e-2) * t28526 + F::new(0.11607361111111111111e-2) * t28529 - F::new(0.17411041666666666666e-2) * t28532 - F::new(0.11607361111111111111e-2) * t28547;
    t28867
}
