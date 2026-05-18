//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1376/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1376<F: Float>(t58397: F, t58401: F, t58405: F, t58409: F, t58412: F, t58415: F, t58418: F, t58421: F, t58424: F, t58428: F, t58431: F, t43503: F, t43508: F, t44329: F, t52446: F, t52452: F, t52591: F, t52593: F, t52596: F, t52601: F, t52687: F, t52689: F, t58435: F) -> (F, F) {
    let t58511 = F::new(0.96922222222222222221e4) * t58397 - F::new(17446.0) * t58401 - F::new(0.14538333333333333333e4) * t58405 + F::new(17446.0) * t58409 + F::new(0.43614999999999999999e4) * t58412 - F::new(0.78666666666666666667e2) * t58415 - F::new(0.94399999999999999998e3) * t58418 - F::new(0.78666666666666666666e2) * t58421 + F::new(1888.0) * t58424 - F::new(0.81580246913580246914e2) * t58428 - F::new(0.21538271604938271605e4) * t58431;
    let t58524 = -F::new(0.72691666666666666667e3) * t58435 + F::new(0.932345679012345679e2) * t52591 - F::new(0.41955555555555555556e3) * t52593 + F::new(0.12586666666666666667e4) * t52596 + F::new(0.20977777777777777778e3) * t52601 + F::new(0.19384444444444444445e4) * t52446 - F::new(0.58153333333333333333e4) * t52452 - F::new(0.19384444444444444445e4) * t43503 + F::new(0.38768888888888888889e4) * t43508 - F::new(0.52444444444444444446e3) * t44329 + F::new(0.20977777777777777778e3) * t52687 - F::new(0.12586666666666666667e4) * t52689;
    (t58511, t58524)
}
