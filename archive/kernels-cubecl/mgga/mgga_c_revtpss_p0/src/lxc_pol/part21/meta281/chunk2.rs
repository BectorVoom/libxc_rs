//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1514/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1514<F: Float>(t10175: F, t3917: F, t10009: F, t10147: F, t10151: F, t10154: F, t10157: F, t10160: F, t10163: F, t10166: F, t10169: F, t10171: F, t1424: F, t1445: F, t213: F, t4071: F, t4078: F, t561: F, t9691: F, t9694: F, t9695: F) -> (F, F) {
    let t10176 = t10175 * t3917;
    let t10178 = -t9691 + t9694 - F::cast_from(0.39029762157531132076e-1_f64) * t9695 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t10009 * t561 - F::cast_from(0.65854491829355115987e0_f64) * t1424 * t10147 - F::cast_from(0.32927245914677557992e-1_f64) * t10151 + F::cast_from(0.32927245914677557992e-1_f64) * t10154 - t10157 + F::cast_from(0.39512695097613069591e1_f64) * t4071 * t4078 - F::cast_from(0.21951497276451705329e-1_f64) * t10160 + F::cast_from(0.19514881078765566038e-2_f64) * t10163 + F::cast_from(0.34697458558045176417e-2_f64) * t10166 + F::cast_from(0.29272321618148349057e-1_f64) * t10169 - F::cast_from(0.19756347548806534796e1_f64) * t10171 * t1445 - F::cast_from(0.58544643236296698113e-1_f64) * t10176;
    (t10176, t10178)
}
