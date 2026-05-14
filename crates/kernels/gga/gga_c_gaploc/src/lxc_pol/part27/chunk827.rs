//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 827/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk827<F: Float>(t568: F, t8827: F, t739: F, t8720: F, t531: F, t3049: F, t769: F, t314: F, t313: F, t1035: F, t2154: F, t1036: F, t1040: F, t1044: F, t1049: F, t1966: F, t2049: F, t2087: F, t2159: F, t2194: F, t2992: F, t3055: F, t3061: F, t317: F, t6018: F, t6021: F, t6024: F, t7584: F, t797: F, t813: F, t833: F, t8797: F, t8803: F, t8806: F, t8809: F, t8816: F, t8819: F, t8822: F) -> (F, F, F) {
    let t8828 = t568 * t8827;
    let t8833 = t739 * t8720;
    let t8834 = t531 * t8833;
    let t8841 = t769 * t3049;
    let t8844 = t314 * t8720;
    let t8845 = t313 * t8844;
    let t8848 = t2154 * t1035;
    let t8851 = -0.23005755572352449806e2 * t7584 * t8797 + 0.23005755572352449806e1 * t6024 * t1049 - 0.1022478025437886658e1 * t1966 * t8803 - 0.62115540045351614476e2 * t2087 * t8806 + 0.61348681526273199482e1 * t833 * t8809 - 0.47667319935800568892e0 * t2049 * t2992 - 0.79445533226334281487e-1 * t1036 * t2159 - 0.1022478025437886658e1 * t813 * t8816 + 0.79445533226334281487e-1 * t797 * t8819 + 0.1022478025437886658e1 * t833 * t8822 - 0.46011511144704899612e1 * t2194 * t3061 - 0.23005755572352449806e1 * t813 * t8828 - 0.71500979903700853338e0 * t2049 * t3055 - 0.35750489951850426669e0 * t797 * t8834 - 0.23005755572352449806e1 * t6021 * t1044 - 0.35750489951850426669e0 * t6018 * t1040 + 0.71500979903700853338e0 * t8841 * t317 + 0.35750489951850426669e0 * t8845 * t317 + 0.35750489951850426669e0 * t8848 * t317;
    (t8833, t8844, t8851)
}
