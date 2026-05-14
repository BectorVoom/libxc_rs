//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1049/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1049<F: Float>(t3334: F, t90: F, t29896: F, t29898: F, t29901: F, t29911: F, t29913: F, t29915: F, t10257: F, t3833: F, t10473: F, t1529: F, t2268: F, t23726: F, t3347: F, t10113: F, t6313: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31614 = 4.0 / 3.0 * t3334 * t90;
    let t31617 = 63.0 / 512.0 * t29896;
    let t31618 = 385.0 / 16384.0 * t29898;
    let t31619 = 147.0 / 1048576.0 * t29901;
    let t31620 = 49.0 / 1048576.0 * t29911;
    let t31621 = 385.0 / 49152.0 * t29913;
    let t31622 = 21.0 / 512.0 * t29915;
    let t31646 = 0.1138200265427045984e0 * t3833 * t10257;
    let t31652 = 0.42682509953514224398e0 * t2268 * t1529 * t10473;
    let t31660 = 0.2276400530854091968e0 * t23726 * t3347;
    let t31662 = 0.7588001769513639893e-1 * t6313 * t10113;
    (t31614, t31617, t31618, t31619, t31620, t31621, t31622, t31646, t31652, t31660, t31662)
}
