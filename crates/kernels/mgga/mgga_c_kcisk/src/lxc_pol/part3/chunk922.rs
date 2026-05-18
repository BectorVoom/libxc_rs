//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 922/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk922<F: Float>(t1235: F, t13629: F, t344: F, t1237: F, t4038: F, t1242: F, t13526: F, t13530: F, t13546: F, t13552: F, t13595: F, t13598: F, t13601: F, t13605: F, t13609: F, t13612: F, t13616: F) -> (F, F, F, F, F) {
    let t13630 = t1235 * t13629;
    let t13632 = F::new(1.0)/pow_3_2::<f64>(t344);
    let t13633 = t4038 * t1237;
    let t13634 = t13632 * t13633;
    let t13636 = t1242 * t13629;
    let t13642 = -F::new(0.65725333333333333332e0) * t13595 + F::new(0.32862666666666666666e0) * t13598 - F::new(0.98587999999999999998e0) * t13601 + F::new(0.10954222222222222222e0) * t13605 - F::new(0.73028148148148148146e-1) * t13609 - F::new(0.16431333333333333333e0) * t13612 - F::new(0.5477111111111111111e0) * t13616 + F::new(0.1898925e1) * t13630 - F::new(0.76790625e-1) * t13634 + F::new(0.3071625e0) * t13636 - F::new(0.59793333333333333333e0) * t13546 + F::new(0.17938e1) * t13552 - F::new(0.39862222222222222223e0) * t13526 + F::new(0.19931111111111111111e0) * t13530;
    (t13630, t13633, t13634, t13636, t13642)
}
