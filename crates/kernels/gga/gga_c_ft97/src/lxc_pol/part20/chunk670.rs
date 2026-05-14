//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 670/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk670<F: Float>(t10697: F, t14602: F, t1248: F, t2801: F, t2843: F, t4299: F, t875: F, t13301: F, t835: F, t3281: F, t13320: F, t2857: F, t1091: F, t2739: F, t2665: F, t446: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14603 = t10697 * t14602;
    let t14607 = t1248 * t2801;
    let t14608 = t2843 * t14607;
    let t14615 = t4299 * t875;
    let t14616 = t2843 * t14615;
    let t14618 = t835 * t13301;
    let t14619 = t3281 * t14618;
    let t14621 = t2857 * t13320;
    let t14622 = t3281 * t14621;
    let t14624 = t1091 * t2739;
    let t14625 = t2665 * t14624;
    let t14626 = t446 * t14625;
    (t14603, t14607, t14608, t14615, t14616, t14619, t14622, t14624, t14626)
}
