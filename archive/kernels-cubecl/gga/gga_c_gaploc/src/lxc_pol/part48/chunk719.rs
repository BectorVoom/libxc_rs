//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 719/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk719<F: Float>(t13625: F, t969: F, t825: F, t13591: F, t13595: F, t13597: F, t13600: F, t13604: F, t13606: F, t13608: F, t13611: F, t13613: F, t13619: F, t13623: F, t2087: F) -> (F, F) {
    let t13626 = t969 * t13625;
    let t13627 = t825 * t13626;
    let t13629 = t13591 - t13595 + t13597 + t13600 - t13604 - t13606 - t13608 + t13611 - F::cast_from(0.13803453343411469884e2_f64) * t2087 * t13613 + t13619 - t13623 - F::cast_from(0.38342925953920749677e0_f64) * t13627;
    (t13626, t13629)
}
