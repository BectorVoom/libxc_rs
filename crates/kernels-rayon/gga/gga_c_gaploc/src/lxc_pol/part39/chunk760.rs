//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 760/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk760(t12922: f64, t8352: f64, t2778: f64, t3116: f64, t1445: f64, t574: f64, t12452: f64, t12456: f64, t12508: f64, t12510: f64, t12512: f64, t9439: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12924 = 0.42900587942220512003e1_f64 * t8352 * t12922;
    let t12925 = t2778 * t3116;
    let t12926 = t1445 * t12925;
    let t12928 = 0.46011511144704899612e1_f64 * t574 * t12926;
    let t12931 = 0.89376224879626066674e-1_f64 * t12452;
    let t12932 = 0.59584149919750711116e-1_f64 * t12456;
    let t12935 = 0.29792074959875355558e-1_f64 * t12508;
    let t12936 = 0.29792074959875355558e-1_f64 * t12510;
    let t12937 = 0.29792074959875355558e-1_f64 * t12512;
    let t12938 = t9439 * t986;
    (t12924, t12925, t12926, t12928, t12931, t12932, t12935, t12936, t12937, t12938)
}
