//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 717/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk717<F: Float>(t11801: F, t13609: F, t3009: F, t3431: F, t1445: F, t11832: F, t935: F, t5748: F, t11894: F, t2087: F, t123: F, t3601: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13611 = F::cast_from(0.42900587942220512003e1_f64) * t11801 * t13609;
    let t13612 = t3009 * t3431;
    let t13613 = t1445 * t13612;
    let t13616 = t11832 * t935;
    let t13617 = t1445 * t13616;
    let t13619 = F::cast_from(0.27606906686822939767e2_f64) * t5748 * t13617;
    let t13620 = t11894 * t935;
    let t13621 = t1445 * t13620;
    let t13623 = F::cast_from(0.69017266717057349418e1_f64) * t2087 * t13621;
    let t13624 = t3601 * t123;
    (t13611, t13612, t13613, t13616, t13617, t13619, t13620, t13621, t13623, t13624)
}
