//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 836/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk836<F: Float>(t1904: F, t5691: F, t22958: F, t5674: F, t1588: F, t22883: F, t28: F, t89: F, t7824: F, t446: F, t432: F, t5617: F, t1800: F, t1317: F, t1307: F, t1755: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22959 = t5691 * t1904;
    let t22960 = t22958 * t22959;
    let t22961 = t5674 * t22960;
    let t22963 = t22883 * t1588;
    let t22964 = t28 * t22963;
    let t22965 = t89 * t22964;
    let t22967 = t7824 * t22959;
    let t22968 = t446 * t22967;
    let t22970 = t5617 * t432;
    let t22971 = t1800 * t22970;
    let t22973 = t1317 * t28 * t22971;
    let t22975 = t1307 * t1755;
    (t22959, t22960, t22961, t22963, t22965, t22967, t22968, t22970, t22971, t22973, t22975)
}
