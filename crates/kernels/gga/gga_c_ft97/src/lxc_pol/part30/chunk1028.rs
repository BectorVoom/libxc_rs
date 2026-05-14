//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1028/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1028<F: Float>(t1882: F, t36253: F, t36195: F, t681: F, t89: F, t36264: F, t36172: F, t36199: F, t112952: F, t1255: F, t143989: F, t152648: F, t152856: F, t15369: F, t1901: F, t24886: F, t2862: F, t29083: F, t29203: F, t29208: F, t29245: F, t29399: F, t296: F, t319: F, t33953: F, t4167: F, t446: F, t53797: F, t54032: F, t6353: F, t840: F) -> (F, F, F, F) {
    let t154591 = t1882 * t36253;
    let t154602 = t89 * t681 * t36195;
    let t154604 = t1882 * t36264;
    let t154626 = t1882 * t36172;
    let t154631 = t1882 * t36199;
    let t154656 = 4.0 / 9.0 * t53797 * t112952 * t29203 - 4.0 / 27.0 * t54032 * t112952 * t29208 + t154626 / 9.0 + 2.0 / 3.0 * t446 * t296 * t152648 + t154631 / 9.0 - t446 * t840 * t1255 * t33953 / 3.0 + 2.0 / 9.0 * t1901 * t24886 * t29083 + 2.0 / 3.0 * t446 * t840 * t6353 * t29245 + 2.0 / 3.0 * t446 * t840 * t6353 * t29399 + 2.0 / 3.0 * t446 * t2862 * t319 * t152856 - 2.0 / 3.0 * t1901 * t15369 * t143989 * t4167;
    (t154591, t154602, t154604, t154656)
}
