//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 842/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk842<F: Float>(t11805: F, t4: F, t11: F, t2: F, t3151: F, t721: F, t228: F, t2682: F, t11799: F, t672: F, t11802: F, t210: F) -> (F, F, F, F, F) {
    let t11806 = t4 * t11805;
    let t11808 = F::powf(t11, -F::cast_from(0.25e1_f64));
    let t11811 = t11808 * t2 * t3151 * t721;
    let t11813 = t2682 * t228;
    let t11815 = t672 * t11799;
    let t11817 = t210 * t11802;
    (t11806, t11811, t11813, t11815, t11817)
}
