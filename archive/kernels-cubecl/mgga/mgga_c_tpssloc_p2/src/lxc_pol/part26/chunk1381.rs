//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1381/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1381<F: Float>(t28: F, t2161: F, t2250: F, t24916: F, t52: F, t607: F, t7402: F, t83655: F, t86534: F, t9258: F, t113: F, t12507: F, t1393: F, t2165: F, t24924: F, t24939: F, t574: F, t652: F, t671: F, t7266: F, t83882: F, t83884: F, t83888: F, t83896: F, t83905: F, t83913: F, t83917: F, t83919: F, t83921: F, t83924: F, t83928: F, t83932: F, t83939: F, t85613: F, t85627: F, t9347: F, t9416: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t86544 = piecewise3::<F>(t401, t83655, t86534 * t52 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24916 * t607 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t7402 * t2250 - t2161 * t9258 / F::cast_from(2.0_f64));
    let t86548 = -F::cast_from(6.0_f64) * t7266 * t12507 - F::cast_from(2.0_f64) * t652 * t2165 * t9416 - F::cast_from(6.0_f64) * t652 * t24924 * t671 + t83882 + t83884 - t83888 - t83896 + t85613 * t574 + F::cast_from(3.0_f64) * t24939 * t1393 + t83905 - t83913 - t83917 - t83919 - t83921 - t83924 - t83928 + t83932 - t83939 - t113 * (t85627 + t86544) - t9347 * t2165;
    t86548
}
