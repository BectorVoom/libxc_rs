//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1313/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1313<F: Float>(t1969: F, t5899: F, t925: F, t95029: F, t26950: F, t379: F, t23667: F, t3051: F, t5889: F, t18: F, t23671: F, t590: F, t5916: F, t13070: F, t40830: F, t5900: F) -> (F, F, F, F, F) {
    let t105386 = t5899 * t1969 * t95029 * t925;
    let t105388 = t26950 * t379;
    let t105390 = t5899 * t23667 * t105388;
    let t105392 = t5889 * t3051;
    let t105396 = t105392 * t23671 * t5916 * t18 * t590;
    let t105400 = t5899 * t40830 * t5900 * t13070;
    (t105386, t105388, t105390, t105396, t105400)
}
