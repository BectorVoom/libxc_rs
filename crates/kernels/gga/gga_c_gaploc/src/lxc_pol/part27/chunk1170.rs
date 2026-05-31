//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1170/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1170<F: Float>(t10205: F, t64: F, t3334: F, t90: F, t29896: F, t29898: F, t29901: F, t29911: F, t29913: F, t29915: F, t10257: F, t3833: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31612 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t10205 * t64;
    let t31614 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3334 * t90;
    let t31617 = F::cast_from(63.0_f64) / F::cast_from(512.0_f64) * t29896;
    let t31618 = F::cast_from(385.0_f64) / F::cast_from(16384.0_f64) * t29898;
    let t31619 = F::cast_from(147.0_f64) / F::cast_from(1048576.0_f64) * t29901;
    let t31620 = F::cast_from(49.0_f64) / F::cast_from(1048576.0_f64) * t29911;
    let t31621 = F::cast_from(385.0_f64) / F::cast_from(49152.0_f64) * t29913;
    let t31622 = F::cast_from(21.0_f64) / F::cast_from(512.0_f64) * t29915;
    let t31646 = F::cast_from(0.1138200265427045984e0_f64) * t3833 * t10257;
    (t31612, t31614, t31617, t31618, t31619, t31620, t31621, t31622, t31646)
}
