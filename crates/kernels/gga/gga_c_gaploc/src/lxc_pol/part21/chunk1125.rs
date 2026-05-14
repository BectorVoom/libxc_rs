//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1125/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1125<F: Float>(t1445: F, t2087: F, t3009: F, t7112: F, t10882: F, t1391: F, t2684: F, t11061: F, t14549: F, t32356: F, t5241: F, t5640: F, t590: F, t1890: F, t1966: F, t32435: F) -> (F, F, F, F, F) {
    let t33505 = 0.69017266717057349418e1 * t2087 * t1445 * t3009 * t7112;
    let t33507 = t2684 * t1391 * t10882;
    let t33508 = 0.2698205900461089792e0 * t33507;
    let t33518 = 0.30674340763136599742e1 * t14549 * t11061;
    let t33522 = 0.30674340763136599742e1 * t5640 * t5241 * t32356 * t590;
    let t33526 = 0.51123901271894332902e1 * t1966 * t1890 * t32435 * t590;
    (t33505, t33508, t33518, t33522, t33526)
}
