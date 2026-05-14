//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 994/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk994<F: Float>(t119849: F, t4500: F, t120090: F, t119891: F, t14686: F, t1579: F, t119968: F, t119836: F, t119888: F, t27279: F, t31854: F, t33711: F, t120082: F, t33716: F, t119935: F, t33674: F) -> (F, F, F, F, F, F, F, F) {
    let t126368 = t119849 * t4500;
    let t126370 = t120090 * t4500;
    let t126375 = t14686 * t119891 * t1579;
    let t126376 = t119968 * t126375;
    let t126378 = t119836 * t126375;
    let t126380 = t119888 * t27279;
    let t126384 = t33711 * t31854;
    let t126386 = t120082 * t33716;
    let t126388 = t119935 * t33674;
    (t126368, t126370, t126376, t126378, t126380, t126384, t126386, t126388)
}
