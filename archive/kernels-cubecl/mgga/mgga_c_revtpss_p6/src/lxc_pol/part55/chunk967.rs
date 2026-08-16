//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 967/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk967<F: Float>(t1310: F, t7983: F, t7315: F, t8108: F, t13648: F, t2107: F, t28683: F, t508: F, t22496: F, t26405: F, t5542: F, t7536: F) -> (F, F, F, F, F, F) {
    let t28704 = t1310 * t7983;
    let t28707 = t8108 * t7315;
    let t28709 = t2107 * t13648;
    let t28711 = t508 * t28683;
    let t28718 = t26405 * t22496;
    let t28727 = t7536 * t5542;
    (t28704, t28707, t28709, t28711, t28718, t28727)
}
