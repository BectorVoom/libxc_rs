//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1136/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1136<F: Float>(t8333: F, t973: F, t2294: F, t2300: F, t8344: F, t970: F, t346: F, t349: F, t8343: F, t2302: F, t2315: F, t23543: F, t23545: F, t23551: F, t23553: F, t23555: F, t23557: F, t23561: F, t23565: F, t23567: F, t23569: F, t23576: F) -> (F, F, F, F, F, F, F) {
    let t23691 = t8333 * t973;
    let t23694 = t2294 * t2300;
    let t23699 = t970 * t8344;
    let t23708 = t346 / t8343 / t349;
    let t23709 = t2302 * t2302;
    let t23715 = t2315 * t2315;
    let t23732 = -F::new(0.17481481481481481482e3) * t23543 - F::new(0.41955555555555555556e3) * t23545 + F::new(0.41955555555555555555e3) * t23551 + F::new(0.93234567901234567903e3) * t23553 + F::new(0.10488888888888888889e4) * t23555 + F::new(0.12586666666666666667e4) * t23557 - F::new(0.94399999999999999998e3) * t23561 - F::new(0.78666666666666666666e2) * t23565 + F::new(0.20977777777777777778e3) * t23567 + F::new(0.932345679012345679e2) * t23569 - F::new(0.81580246913580246914e2) * t23576;
    (t23691, t23694, t23699, t23708, t23709, t23715, t23732)
}
