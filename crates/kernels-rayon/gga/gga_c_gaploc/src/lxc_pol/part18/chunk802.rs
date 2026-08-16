//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 802/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk802(t568: f64, t7607: f64, t531: f64, t7124: f64, t2017: f64, t913: f64, t825: f64, t2049: f64, t2194: f64, t2197: f64, t2699: f64, t2705: f64, t2711: f64, t2714: f64, t2718: f64, t317: f64, t6018: f64, t6021: f64, t6024: f64, t7580: f64, t7582: f64, t7584: f64, t7586: f64, t7590: f64, t7593: f64, t7596: f64, t7602: f64, t784: f64, t797: f64, t813: f64, t833: f64, t962: f64, t966: f64, t974: f64) -> (f64, f64) {
    let t7608 = t568 * t7607;
    let t7615 = t531 * t7124;
    let t7626 = t2017 * t913;
    let t7627 = t825 * t7626;
    let t7629 = -0.95857314884801874192e-1_f64 * t7580 + 0.42603251059911944086e-1_f64 * t7582 - 0.23005755572352449806e2_f64 * t7584 * t7586 + 0.35750489951850426669e0_f64 * t7590 * t317 + 0.35750489951850426669e0_f64 * t7593 * t317 + 0.71500979903700853338e0_f64 * t7596 * t317 + 0.46011511144704899612e1_f64 * t2197 * t2711 + 0.23005755572352449806e1_f64 * t833 * t7602 - 0.46011511144704899612e1_f64 * t2194 * t2705 - 0.23005755572352449806e1_f64 * t813 * t7608 + 0.23005755572352449806e1_f64 * t6024 * t974 - 0.71500979903700853338e0_f64 * t2049 * t2699 - 0.35750489951850426669e0_f64 * t797 * t7615 - 0.23005755572352449806e1_f64 * t6021 * t966 - 0.35750489951850426669e0_f64 * t6018 * t962 + 0.47667319935800568892e0_f64 * t2714 * t784 + 0.47667319935800568892e0_f64 * t2718 * t784 - 0.29822275741938360861e0_f64 * t7627;
    (t7627, t7629)
}
