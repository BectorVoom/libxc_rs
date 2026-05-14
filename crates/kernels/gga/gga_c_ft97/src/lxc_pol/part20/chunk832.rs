//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 832/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk832<F: Float>(t10447: F, t6361: F, t2801: F, t6353: F, t296: F, t2749: F, t6386: F, t1882: F, t6371: F, t2867: F, t840: F, t1508: F, t2413: F, t835: F, t2405: F, t2857: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25287 = t10447 * t6361;
    let t25290 = t6353 * t2801;
    let t25291 = t296 * t25290;
    let t25294 = t2749 * t6386;
    let t25295 = t296 * t25294;
    let t25298 = t1882 * t6371;
    let t25301 = t840 * t6353 * t2867;
    let t25305 = t835 * t1508 * t2413;
    let t25309 = t2857 * t1508 * t2405;
    (t25287, t25290, t25291, t25294, t25295, t25298, t25301, t25305, t25309)
}
