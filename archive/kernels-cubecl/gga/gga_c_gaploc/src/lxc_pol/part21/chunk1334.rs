//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1334/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1334<F: Float>(t30907: F, t30923: F, t30927: F, t10439: F, t1407: F, t2464: F, t2465: F, t587: F, t7980: F, t1352: F, t191: F, t2876: F, t6604: F) -> (F, F, F, F, F, F) {
    let t34774 = F::cast_from(0.95857314884801874192e-1_f64) * t30907;
    let t34775 = F::cast_from(0.31952438294933958064e-1_f64) * t30923;
    let t34776 = F::cast_from(0.12780975317973583226e0_f64) * t30927;
    let t34782 = t1407 * t10439;
    let t34783 = F::cast_from(0.85206502119823888168e-1_f64) * t34782;
    let t34789 = t587 * t2464 * t2465 * t7980;
    let t34790 = F::cast_from(0.85206502119823888168e-1_f64) * t34789;
    let t34794 = F::cast_from(0.71500979903700853338e0_f64) * t6604 * t2876 * t191 * t1352;
    (t34774, t34775, t34776, t34783, t34790, t34794)
}
