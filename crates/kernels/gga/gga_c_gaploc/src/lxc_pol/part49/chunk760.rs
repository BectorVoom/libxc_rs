//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 760/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk760<F: Float>(t12922: F, t8352: F, t2778: F, t3116: F, t1445: F, t574: F, t12452: F, t12456: F, t12508: F, t12510: F, t12512: F, t9439: F, t986: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12924 = F::cast_from(0.42900587942220512003e1_f64) * t8352 * t12922;
    let t12925 = t2778 * t3116;
    let t12926 = t1445 * t12925;
    let t12928 = F::cast_from(0.46011511144704899612e1_f64) * t574 * t12926;
    let t12931 = F::cast_from(0.89376224879626066674e-1_f64) * t12452;
    let t12932 = F::cast_from(0.59584149919750711116e-1_f64) * t12456;
    let t12935 = F::cast_from(0.29792074959875355558e-1_f64) * t12508;
    let t12936 = F::cast_from(0.29792074959875355558e-1_f64) * t12510;
    let t12937 = F::cast_from(0.29792074959875355558e-1_f64) * t12512;
    let t12938 = t9439 * t986;
    (t12924, t12925, t12926, t12928, t12931, t12932, t12935, t12936, t12937, t12938)
}
