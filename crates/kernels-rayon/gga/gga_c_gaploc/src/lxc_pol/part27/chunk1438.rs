//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1438/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1438(t12176: f64, t12185: f64, t12214: f64, t12231: f64, t1445: f64, t1457: f64, t1998: f64, t2154: f64, t2194: f64, t313: f64, t314: f64, t317: f64, t33604: f64, t33607: f64, t33610: f64, t33613: f64, t3726: f64, t3732: f64, t3736: f64, t38975: f64, t39044: f64, t39107: f64, t39181: f64, t4585: f64, t4614: f64, t568: f64, t6018: f64, t7572: f64, t7573: f64, t769: f64, t797: f64, t807: f64, t808: f64, t813: f64, t833: f64) -> f64 {
    let t39246 = -0.61348681526273199482e1_f64 * t1998 * t4614 * t12231 - 0.21450293971110256002e1_f64 * t797 * t1457 * t38975 + 0.46011511144704899612e1_f64 * t807 * t1445 * t39107 + 0.30674340763136599741e2_f64 * t833 * t4614 * t12214 + 0.79445533226334281487e-1_f64 * t797 * t4585 * t3726 - 0.61348681526273199482e1_f64 * t2194 * t12185 + 0.35750489951850426669e0_f64 * t2154 * t3732 * t317 - 0.35750489951850426669e0_f64 * t6018 * t3736 - 0.23005755572352449806e1_f64 * t813 * t568 * t808 * t39181 + 0.71500979903700853338e0_f64 * t769 * t12176 * t317 + 0.35750489951850426669e0_f64 * t313 * t314 * t39181 * t317 + 0.13803453343411469884e2_f64 * t7572 * t7573 * t39044 - t33604 + t33607 + t33610 - t33613;
    t39246
}
