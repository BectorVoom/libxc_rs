//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1387/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1387<F: Float>(t27759: F, t27761: F, t28302: F, t27153: F, t27154: F, t27157: F, t27159: F, t8: F, t93848: F, t93849: F, t93852: F, t97547: F, t97567: F, t97585: F, t97602: F) -> F {
    let t97606 = t27759 / F::new(8.0);
    let t97607 = t27761 / F::new(8.0);
    let t97608 = t28302 / F::new(8.0);
    let t97609 = t8 * (t97547 + t97567 + t97585 + t97602) - t97606 + t97607 - t27153 + t27154 + t93848 - t93849 - t27157 - t97608 + t27159 + t93852;
    t97609
}
