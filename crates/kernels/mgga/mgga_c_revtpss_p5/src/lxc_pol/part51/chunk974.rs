//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 974/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk974<F: Float>(t7003: F, t8634: F, t196: F, t197: F, t7231: F, t2035: F, t6985: F, t7313: F, t8568: F, t32171: F, t508: F, t1310: F, t8454: F) -> (F, F, F, F, F, F, F) {
    let t32320 = F::cast_from(4.0_f64) * t8634 * t7003;
    let t32322 = t7231 * t196 * t197;
    let t32323 = t32322 * t2035;
    let t32325 = t6985 * t7003;
    let t32329 = t8568 * t7313;
    let t32338 = F::cast_from(2.0_f64) * t32171 * t508;
    let t32340 = F::cast_from(2.0_f64) * t8454 * t1310;
    (t32320, t32322, t32323, t32325, t32329, t32338, t32340)
}
