//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 432/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk432(t2060: f64, t313: f64, t2018: f64, t2022: f64, t2023: f64, t2027: f64, t2028: f64, t2033: f64, t2034: f64, t2037: f64, t2043: f64, t2046: f64, t2049: f64, t2053: f64, t2054: f64, t2057: f64, t317: f64, t775: f64, t780: f64, t784: f64, t797: f64, t798: f64, t807: f64, t825: f64) -> (f64, f64) {
    let t2061 = t313 * t2060;
    let t2064 = -0.2556195063594716645e0_f64 * t825 * t2018 + 0.79445533226334281486e-1_f64 * t2022 * t2023 - 0.79445533226334281486e-1_f64 * t2027 * t2028 + 0.79445533226334281486e-1_f64 * t2033 * t2034 + 0.30674340763136599742e1_f64 * t807 * t2037 + 0.47667319935800568892e0_f64 * t780 * t784 + 0.35750489951850426669e0_f64 * t2043 * t317 - 0.47667319935800568892e0_f64 * t797 * t2046 - 0.71500979903700853338e0_f64 * t2049 * t798 + 0.71500979903700853338e0_f64 * t2053 * t2054 - 0.47667319935800568892e0_f64 * t775 * t2057 + 0.35750489951850426669e0_f64 * t2061 * t317;
    (t2061, t2064)
}
