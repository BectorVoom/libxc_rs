//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1437/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1437<F: Float>(t12176: F, t12185: F, t12214: F, t12231: F, t1445: F, t1457: F, t1998: F, t2154: F, t2194: F, t313: F, t314: F, t317: F, t33604: F, t33607: F, t33610: F, t33613: F, t3726: F, t3732: F, t3736: F, t38975: F, t39044: F, t39107: F, t39181: F, t4585: F, t4614: F, t568: F, t6018: F, t7572: F, t7573: F, t769: F, t797: F, t807: F, t808: F, t813: F, t833: F) -> F {
    let t39246 = -F::cast_from(0.61348681526273199482e1_f64) * t1998 * t4614 * t12231 - F::cast_from(0.21450293971110256002e1_f64) * t797 * t1457 * t38975 + F::cast_from(0.46011511144704899612e1_f64) * t807 * t1445 * t39107 + F::cast_from(0.30674340763136599741e2_f64) * t833 * t4614 * t12214 + F::cast_from(0.79445533226334281487e-1_f64) * t797 * t4585 * t3726 - F::cast_from(0.61348681526273199482e1_f64) * t2194 * t12185 + F::cast_from(0.35750489951850426669e0_f64) * t2154 * t3732 * t317 - F::cast_from(0.35750489951850426669e0_f64) * t6018 * t3736 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t568 * t808 * t39181 + F::cast_from(0.71500979903700853338e0_f64) * t769 * t12176 * t317 + F::cast_from(0.35750489951850426669e0_f64) * t313 * t314 * t39181 * t317 + F::cast_from(0.13803453343411469884e2_f64) * t7572 * t7573 * t39044 - t33604 + t33607 + t33610 - t33613;
    t39246
}
