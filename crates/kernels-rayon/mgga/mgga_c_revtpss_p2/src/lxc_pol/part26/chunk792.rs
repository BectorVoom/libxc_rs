//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 792/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk792(t10174: f64, t786: f64, t3917: f64, t10009: f64, t10147: f64, t10151: f64, t10154: f64, t10157: f64, t10160: f64, t10163: f64, t10166: f64, t10169: f64, t10171: f64, t1424: f64, t1445: f64, t213: f64, t4071: f64, t4078: f64, t561: f64, t9691: f64, t9694: f64, t9695: f64) -> f64 {
    let t10175 = t786 * t10174;
    let t10176 = t10175 * t3917;
    let t10178 = -t9691 + t9694 - 0.39029762157531132076e-1_f64 * t9695 + 0.65854491829355115987e0_f64 * t213 * t10009 * t561 - 0.65854491829355115987e0_f64 * t1424 * t10147 - 0.32927245914677557992e-1_f64 * t10151 + 0.32927245914677557992e-1_f64 * t10154 - t10157 + 0.39512695097613069591e1_f64 * t4071 * t4078 - 0.21951497276451705329e-1_f64 * t10160 + 0.19514881078765566038e-2_f64 * t10163 + 0.34697458558045176417e-2_f64 * t10166 + 0.29272321618148349057e-1_f64 * t10169 - 0.19756347548806534796e1_f64 * t10171 * t1445 - 0.58544643236296698113e-1_f64 * t10176;
    t10178
}
