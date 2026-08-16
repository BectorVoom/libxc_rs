//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 962/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk962(t209: f64, t49834: f64, t49851: f64, t49859: f64, t49891: f64, t49907: f64, t49942: f64, t49958: f64, t49961: f64, t1382: f64, t2902: f64, t3718: f64) -> (f64, f64) {
    let t49965 = (t49834 + t49851 + t49859 + t49891 + t49907 + t49942 + t49958 + t49961) * t209;
    let t49968 = 4.0_f64 * t1382 * t2902 * t3718;
    (t49965, t49968)
}
