//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 903/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk903<F: Float>(t102: F, t8416: F, t100: F, t480: F, t8417: F, t1786: F, t1825: F, t24: F, t32075: F, t1852: F, t488: F, t8216: F) -> (F, F, F, F, F, F, F) {
    let t38651 = F::cast_from(1.0_f64) / t8416 / t102;
    let t38652 = t100 * t38651;
    let t38659 = t480 * t8417;
    let t38711 = t1786 * t1825;
    let t38921 = t24 * t32075;
    let t39107 = t1786 * t1852;
    let t39120 = t8216 * t488;
    (t38651, t38652, t38659, t38711, t38921, t39107, t39120)
}
