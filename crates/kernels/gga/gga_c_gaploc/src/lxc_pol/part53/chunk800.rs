//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 800/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk800<F: Float>(t1991: F, t43838: F, t590: F, t1890: F, t1966: F, t43107: F, t13012: F, t2087: F, t4614: F, t3267: F, t8634: F, t13033: F, t5748: F, t42944: F, t739: F, t11068: F, t2617: F, t7803: F) -> (F, F, F, F, F, F, F, F) {
    let t43841 = 0.1022478025437886658e1 * t1991 * t43838 * t590;
    let t43849 = 0.25561950635947166451e1 * t1966 * t1890 * t43107 * t590;
    let t43858 = 0.92023022289409799224e1 * t2087 * t4614 * t13012;
    let t43861 = 0.35750489951850426669e0 * t3267 * t8634;
    let t43864 = 0.36809208915763919689e2 * t5748 * t4614 * t13033;
    let t43875 = 0.20449560508757733161e1 * t1991 * t739 * t42944 * t590;
    let t43879 = 0.97135412416599232513e1 * t1966 * t1890 * t42944 * t590;
    let t43881 = t7803 * t11068 * t2617;
    (t43841, t43849, t43858, t43861, t43864, t43875, t43879, t43881)
}
