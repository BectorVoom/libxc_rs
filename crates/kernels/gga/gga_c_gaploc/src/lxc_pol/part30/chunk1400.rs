//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1400/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1400<F: Float>(t2464: F, t2465: F, t587: F, t7980: F, t1352: F, t191: F, t2876: F, t6604: F, t1411: F, t3395: F, t2365: F, t2366: F, t4379: F, t7892: F) -> (F, F, F, F) {
    let t34789 = t587 * t2464 * t2465 * t7980;
    let t34790 = F::cast_from(0.85206502119823888168e-1_f64) * t34789;
    let t34794 = F::cast_from(0.71500979903700853338e0_f64) * t6604 * t2876 * t191 * t1352;
    let t34796 = t587 * t1411 * t3395;
    let t34797 = F::cast_from(0.59644551483876721719e0_f64) * t34796;
    let t34800 = t4379 * t2365 * t2366 * t7892;
    (t34790, t34794, t34797, t34800)
}
