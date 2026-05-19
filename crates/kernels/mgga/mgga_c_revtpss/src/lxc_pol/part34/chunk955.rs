//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 955/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk955<F: Float>(t23019: F, t23041: F, t1427: F, t10157: F, t14091: F, t14097: F, t14105: F, t1424: F, t14280: F, t14290: F, t14294: F, t14297: F, t1904: F, t22390: F, t22428: F, t22447: F, t22450: F, t22454: F, t5715: F, t6919: F) -> (F, F, F) {
    let t23042 = t23019 + t23041;
    let t23043 = t1427 * t23042;
    let t23058 = F::cast_from(0.39029762157531132076e-1_f64) * t14091 + F::cast_from(0.21951497276451705329e-1_f64) * t14097 - F::cast_from(0.34697458558045176417e-2_f64) * t14105 - F::cast_from(0.65854491829355115987e0_f64) * t1424 * t23043 + F::cast_from(0.32927245914677557992e-1_f64) * t22428 - F::cast_from(0.19756347548806534796e1_f64) * t5715 * t6919 - t10157 - F::cast_from(0.39029762157531132076e-1_f64) * t14280 - F::cast_from(0.19756347548806534796e1_f64) * t22390 * t1904 - F::cast_from(0.16463622957338778996e-1_f64) * t22447 - F::cast_from(0.32927245914677557992e-1_f64) * t22450 + F::cast_from(0.58544643236296698113e-1_f64) * t22454 - F::cast_from(0.21951497276451705329e-1_f64) * t14290 + F::cast_from(0.34697458558045176417e-2_f64) * t14294 + F::cast_from(0.19514881078765566038e-2_f64) * t14297;
    (t23042, t23043, t23058)
}
