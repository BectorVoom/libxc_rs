//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 756/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk756<F: Float>(t7626: F, t825: F, t2049: F, t2194: F, t2197: F, t2699: F, t2705: F, t2711: F, t2714: F, t2718: F, t317: F, t6018: F, t6021: F, t6024: F, t7580: F, t7582: F, t7584: F, t7586: F, t7590: F, t7593: F, t7596: F, t7602: F, t7608: F, t7615: F, t784: F, t797: F, t813: F, t833: F, t962: F, t966: F, t974: F) -> (F, F) {
    let t7627 = t825 * t7626;
    let t7629 = -0.95857314884801874192e-1 * t7580 + 0.42603251059911944086e-1 * t7582 - 0.23005755572352449806e2 * t7584 * t7586 + 0.35750489951850426669e0 * t7590 * t317 + 0.35750489951850426669e0 * t7593 * t317 + 0.71500979903700853338e0 * t7596 * t317 + 0.46011511144704899612e1 * t2197 * t2711 + 0.23005755572352449806e1 * t833 * t7602 - 0.46011511144704899612e1 * t2194 * t2705 - 0.23005755572352449806e1 * t813 * t7608 + 0.23005755572352449806e1 * t6024 * t974 - 0.71500979903700853338e0 * t2049 * t2699 - 0.35750489951850426669e0 * t797 * t7615 - 0.23005755572352449806e1 * t6021 * t966 - 0.35750489951850426669e0 * t6018 * t962 + 0.47667319935800568892e0 * t2714 * t784 + 0.47667319935800568892e0 * t2718 * t784 - 0.29822275741938360861e0 * t7627;
    (t7627, t7629)
}
