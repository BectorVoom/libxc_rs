//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 803/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk803<F: Float>(t568: F, t7607: F, t531: F, t7124: F, t2017: F, t913: F, t825: F, t2049: F, t2194: F, t2197: F, t2699: F, t2705: F, t2711: F, t2714: F, t2718: F, t317: F, t6018: F, t6021: F, t6024: F, t7580: F, t7582: F, t7584: F, t7586: F, t7590: F, t7593: F, t7596: F, t7602: F, t784: F, t797: F, t813: F, t833: F, t962: F, t966: F, t974: F) -> (F, F) {
    let t7608 = t568 * t7607;
    let t7615 = t531 * t7124;
    let t7626 = t2017 * t913;
    let t7627 = t825 * t7626;
    let t7629 = -F::cast_from(0.95857314884801874192e-1_f64) * t7580 + F::cast_from(0.42603251059911944086e-1_f64) * t7582 - F::cast_from(0.23005755572352449806e2_f64) * t7584 * t7586 + F::cast_from(0.35750489951850426669e0_f64) * t7590 * t317 + F::cast_from(0.35750489951850426669e0_f64) * t7593 * t317 + F::cast_from(0.71500979903700853338e0_f64) * t7596 * t317 + F::cast_from(0.46011511144704899612e1_f64) * t2197 * t2711 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t7602 - F::cast_from(0.46011511144704899612e1_f64) * t2194 * t2705 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t7608 + F::cast_from(0.23005755572352449806e1_f64) * t6024 * t974 - F::cast_from(0.71500979903700853338e0_f64) * t2049 * t2699 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t7615 - F::cast_from(0.23005755572352449806e1_f64) * t6021 * t966 - F::cast_from(0.35750489951850426669e0_f64) * t6018 * t962 + F::cast_from(0.47667319935800568892e0_f64) * t2714 * t784 + F::cast_from(0.47667319935800568892e0_f64) * t2718 * t784 - F::cast_from(0.29822275741938360861e0_f64) * t7627;
    (t7627, t7629)
}
