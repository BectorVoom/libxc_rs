//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1254/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1254<F: Float>(t12213: F, t12223: F, t1391: F, t1392: F, t2684: F, t28249: F, t28259: F, t28281: F, t32968: F, t32972: F, t32973: F, t32974: F, t32979: F, t32984: F, t32987: F, t32991: F, t32997: F, t33001: F, t33004: F, t825: F) -> (F,) {
    let t38993 = 0.11360866949309851756e0 * t2684 * t1391 * t1392 * t12213 - 0.11360866949309851756e0 * t825 * t1391 * t1392 * t12223 - t32968 - t32972 - t32973 + t32974 - t28249 - t28259 - t32979 - t32984 - t32987 + t32991 + t32997 - t33001 + t33004 + t28281;
    (t38993,)
}
