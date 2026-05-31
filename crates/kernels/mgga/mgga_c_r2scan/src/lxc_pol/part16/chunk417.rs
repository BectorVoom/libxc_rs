//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 417/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk417<F: Float>(t1821: F, t224: F, t1691: F, t712: F, t720: F, t695: F, t124: F, t219: F, t201: F, t200: F, t685: F, t63: F) -> (F, F, F, F, F, F, F, F) {
    let t1982 = t224 * t1821;
    let t1983 = t1982 * t1691;
    let t1986 = t712 * t720;
    let t1987 = t1986 * t695;
    let t1990 = t124 * t219;
    let t2000 = t124 * t201;
    let t2005 = F::cast_from(1.0_f64) / t685 / t200;
    let t2006 = t63 * t2005;
    (t1982, t1983, t1986, t1987, t1990, t2000, t2005, t2006)
}
