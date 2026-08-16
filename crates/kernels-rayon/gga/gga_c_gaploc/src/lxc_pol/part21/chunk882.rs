//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 882/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk882(t1036: f64, t1040: f64, t1044: f64, t1049: f64, t1966: f64, t2049: f64, t2087: f64, t2159: f64, t2194: f64, t2992: f64, t3055: f64, t3061: f64, t317: f64, t6018: f64, t6021: f64, t6024: f64, t7584: f64, t797: f64, t813: f64, t833: f64, t8797: f64, t8803: f64, t8806: f64, t8809: f64, t8816: f64, t8819: f64, t8822: f64, t8828: f64, t8834: f64, t8841: f64, t8845: f64, t8848: f64) -> f64 {
    let t8851 = -0.23005755572352449806e2_f64 * t7584 * t8797 + 0.23005755572352449806e1_f64 * t6024 * t1049 - 0.1022478025437886658e1_f64 * t1966 * t8803 - 0.62115540045351614476e2_f64 * t2087 * t8806 + 0.61348681526273199482e1_f64 * t833 * t8809 - 0.47667319935800568892e0_f64 * t2049 * t2992 - 0.79445533226334281487e-1_f64 * t1036 * t2159 - 0.1022478025437886658e1_f64 * t813 * t8816 + 0.79445533226334281487e-1_f64 * t797 * t8819 + 0.1022478025437886658e1_f64 * t833 * t8822 - 0.46011511144704899612e1_f64 * t2194 * t3061 - 0.23005755572352449806e1_f64 * t813 * t8828 - 0.71500979903700853338e0_f64 * t2049 * t3055 - 0.35750489951850426669e0_f64 * t797 * t8834 - 0.23005755572352449806e1_f64 * t6021 * t1044 - 0.35750489951850426669e0_f64 * t6018 * t1040 + 0.71500979903700853338e0_f64 * t8841 * t317 + 0.35750489951850426669e0_f64 * t8845 * t317 + 0.35750489951850426669e0_f64 * t8848 * t317;
    t8851
}
