//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1082/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1082<F: Float>(t2003: F, t465: F, t53: F, t5633: F, t2002: F, t220: F, t310: F, t5999: F, t5952: F, t785: F, t2021: F, t296: F) -> (F, F, F, F, F, F) {
    let t18199 = t465 * t2003;
    let t18204 = t53 * t5633;
    let t18210 = F::new(1.0) / t2002 / t220;
    let t18258 = F::new(1.0) / t5999 / t310;
    let t18278 = t5952 * t785;
    let t18290 = F::new(1.0) / t2021 / t296;
    (t18199, t18204, t18210, t18258, t18278, t18290)
}
