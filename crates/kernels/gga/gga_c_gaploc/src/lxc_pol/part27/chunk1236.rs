//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1236/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1236<F: Float>(t12065: F, t12093: F, t1445: F, t1549: F, t1555: F, t1646: F, t31178: F, t34928: F, t34931: F, t34935: F, t34937: F, t34939: F, t34941: F, t34944: F, t34947: F, t34950: F, t34953: F, t3701: F, t38314: F, t38436: F, t4418: F, t531: F, t557: F, t574: F) -> (F,) {
    let t38801 = t31178 - 0.35750489951850426669e0 * t557 * t531 * t38314 - t34928 + t34931 + t34935 - t34937 + 0.51123901271894332905e0 * t4418 * t12093 + t34939 - t34941 + t34944 - t34947 + t34950 + t34953 - 0.92023022289409799224e1 * t574 * t1445 * t38436 - 0.71500979903700853338e0 * t1555 * t3701 * t1646 + 0.71500979903700853338e0 * t1549 * t12065;
    (t38801,)
}
