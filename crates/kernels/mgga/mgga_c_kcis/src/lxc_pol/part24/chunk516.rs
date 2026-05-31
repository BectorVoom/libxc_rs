//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 516/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk516<F: Float>(t4685: F, t951: F, t1680: F, t2933: F, t949: F, t2938: F, t1670: F, t2960: F, t934: F, t4625: F, t939: F, t1676: F, t659: F) -> (F, F, F, F, F, F, F, F) {
    let t4687 = F::cast_from(1.0_f64) * t4685 * t951;
    let t4689 = F::cast_from(1.0_f64) * t2933 * t1680;
    let t4690 = t1680 * t949;
    let t4692 = F::cast_from(2.0_f64) * t2938 * t4690;
    let t4700 = t2960 * t1670;
    let t4701 = t4700 * t934;
    let t4703 = t939 * t4625;
    let t4706 = t659 * t1676;
    (t4687, t4689, t4690, t4692, t4700, t4701, t4703, t4706)
}
