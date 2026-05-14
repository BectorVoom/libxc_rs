//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 698/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk698<F: Float>(t139: F, t16887: F, t527: F, t1008: F, t132: F, t1013: F, t12367: F, t1995: F, t4699: F, t4703: F, t542: F, t4698: F, t549: F, t554: F, t2057: F, t4702: F) -> (F, F, F, F, F, F, F) {
    let t16888 = t139 * t16887;
    let t16889 = t527 * t16888;
    let t16891 = t1008 * t132;
    let t16894 = t12367 * t1013;
    let t16897 = t1995 * t4699;
    let t16902 = t542 * t4703;
    let t16907 = t549 * t4698;
    let t16908 = t16907 * t554;
    let t16911 = t2057 * t4702;
    (t16889, t16891, t16894, t16897, t16902, t16908, t16911)
}
