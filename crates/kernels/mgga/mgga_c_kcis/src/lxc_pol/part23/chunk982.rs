//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 982/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk982<F: Float>(t12844: F, t6172: F, t4439: F, t531: F, t6183: F, t833: F, t4440: F, t2645: F, t6171: F, t1444: F, t2104: F, t2642: F) -> (F, F, F, F) {
    let t18091 = t12844 * t6172;
    let t18093 = t4439 * t18091 / F::new(864.0);
    let t18094 = t6183 * t531;
    let t18095 = t18094 * t833;
    let t18096 = t4440 * t18095;
    let t18099 = t6171 * t2645;
    let t18100 = t4440 * t18099;
    let t18103 = t2104 * t1444;
    let t18104 = t18103 * t2642;
    (t18093, t18096, t18100, t18104)
}
