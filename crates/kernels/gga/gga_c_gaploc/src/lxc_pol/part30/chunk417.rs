//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 417/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk417<F: Float>(t2060: F, t313: F, t2018: F, t2022: F, t2023: F, t2027: F, t2028: F, t2033: F, t2034: F, t2037: F, t2043: F, t2046: F, t2049: F, t2053: F, t2054: F, t2057: F, t317: F, t775: F, t780: F, t784: F, t797: F, t798: F, t807: F, t825: F) -> (F, F) {
    let t2061 = t313 * t2060;
    let t2064 = -0.2556195063594716645e0 * t825 * t2018 + 0.79445533226334281486e-1 * t2022 * t2023 - 0.79445533226334281486e-1 * t2027 * t2028 + 0.79445533226334281486e-1 * t2033 * t2034 + 0.30674340763136599742e1 * t807 * t2037 + 0.47667319935800568892e0 * t780 * t784 + 0.35750489951850426669e0 * t2043 * t317 - 0.47667319935800568892e0 * t797 * t2046 - 0.71500979903700853338e0 * t2049 * t798 + 0.71500979903700853338e0 * t2053 * t2054 - 0.47667319935800568892e0 * t775 * t2057 + 0.35750489951850426669e0 * t2061 * t317;
    (t2061, t2064)
}
