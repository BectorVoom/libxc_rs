//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 885/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk885<F: Float>(t1036: F, t1040: F, t1044: F, t1049: F, t1966: F, t2049: F, t2087: F, t2159: F, t2194: F, t2992: F, t3055: F, t3061: F, t317: F, t6018: F, t6021: F, t6024: F, t7584: F, t797: F, t813: F, t833: F, t8797: F, t8803: F, t8806: F, t8809: F, t8816: F, t8819: F, t8822: F, t8828: F, t8834: F, t8841: F, t8845: F, t8848: F) -> F {
    let t8851 = -F::new(0.23005755572352449806e2) * t7584 * t8797 + F::new(0.23005755572352449806e1) * t6024 * t1049 - F::new(0.1022478025437886658e1) * t1966 * t8803 - F::new(0.62115540045351614476e2) * t2087 * t8806 + F::new(0.61348681526273199482e1) * t833 * t8809 - F::new(0.47667319935800568892e0) * t2049 * t2992 - F::new(0.79445533226334281487e-1) * t1036 * t2159 - F::new(0.1022478025437886658e1) * t813 * t8816 + F::new(0.79445533226334281487e-1) * t797 * t8819 + F::new(0.1022478025437886658e1) * t833 * t8822 - F::new(0.46011511144704899612e1) * t2194 * t3061 - F::new(0.23005755572352449806e1) * t813 * t8828 - F::new(0.71500979903700853338e0) * t2049 * t3055 - F::new(0.35750489951850426669e0) * t797 * t8834 - F::new(0.23005755572352449806e1) * t6021 * t1044 - F::new(0.35750489951850426669e0) * t6018 * t1040 + F::new(0.71500979903700853338e0) * t8841 * t317 + F::new(0.35750489951850426669e0) * t8845 * t317 + F::new(0.35750489951850426669e0) * t8848 * t317;
    t8851
}
