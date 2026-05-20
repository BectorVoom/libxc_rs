//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 750/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk750<F: Float>(t1424: F, t4071: F, t4132: F, t9632: F, t9636: F, t9639: F, t9642: F, t9650: F, t9652: F, t9659: F, t9666: F, t9668: F, t9672: F, t9677: F, t9683: F, t9687: F) -> F {
    let t9689 = F::cast_from(0.21951497276451705329e-1_f64) * t9632 - F::cast_from(0.16463622957338778996e-1_f64) * t9636 + t9639 - F::cast_from(0.19514881078765566038e-2_f64) * t9642 + t9650 + F::cast_from(0.39512695097613069591e1_f64) * t1424 * t9652 - F::cast_from(0.39512695097613069591e1_f64) * t1424 * t9659 - F::cast_from(0.19756347548806534796e1_f64) * t4071 * t4132 - t9666 + F::cast_from(0.16463622957338778996e-1_f64) * t9668 - F::cast_from(0.29272321618148349057e-1_f64) * t9672 - F::cast_from(0.34697458558045176417e-2_f64) * t9677 + F::cast_from(0.58544643236296698113e-1_f64) * t9683 + F::cast_from(0.39029762157531132076e-1_f64) * t9687;
    t9689
}
