//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2492/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2492<F: Float>(t3651: F, t3655: F, t43813: F, t1256: F, t12890: F, t1222: F, t3693: F, t697: F, t13021: F, t140: F, t12256: F, t3698: F) -> (F, F, F, F, F, F) {
    let t44293 = t3651 * t3655;
    let t44307 = F::cast_from(0.86419753086419753087e-1_f64) * t43813;
    let t44326 = t12890 * t1256;
    let t44343 = t1222 * t697 * t3693;
    let t44346 = t1222 * t140 * t13021;
    let t44348 = t3698 * t12256;
    (t44293, t44307, t44326, t44343, t44346, t44348)
}
