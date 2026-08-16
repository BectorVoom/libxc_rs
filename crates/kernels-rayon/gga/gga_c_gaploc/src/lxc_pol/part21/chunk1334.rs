//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1334/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1334(t30907: f64, t30923: f64, t30927: f64, t10439: f64, t1407: f64, t2464: f64, t2465: f64, t587: f64, t7980: f64, t1352: f64, t191: f64, t2876: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34774 = 0.95857314884801874192e-1_f64 * t30907;
    let t34775 = 0.31952438294933958064e-1_f64 * t30923;
    let t34776 = 0.12780975317973583226e0_f64 * t30927;
    let t34782 = t1407 * t10439;
    let t34783 = 0.85206502119823888168e-1_f64 * t34782;
    let t34789 = t587 * t2464 * t2465 * t7980;
    let t34790 = 0.85206502119823888168e-1_f64 * t34789;
    let t34794 = 0.71500979903700853338e0_f64 * t6604 * t2876 * t191 * t1352;
    (t34774, t34775, t34776, t34783, t34790, t34794)
}
