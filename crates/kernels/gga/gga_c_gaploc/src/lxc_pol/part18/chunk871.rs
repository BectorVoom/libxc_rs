//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 871/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk871<F: Float>(t739: F, t8669: F, t590: F, t1890: F, t5241: F, t8502: F, t1986: F, t1991: F, t1998: F, t2004: F, t2009: F, t2061: F, t2087: F, t2178: F, t2194: F, t2639: F, t3019: F, t3025: F, t3028: F, t3040: F, t3046: F, t5640: F, t5840: F, t7736: F, t780: F, t784: F, t813: F, t8629: F, t8634: F, t8638: F, t8646: F, t8650: F, t8655: F, t8658: F, t8663: F, t8666: F) -> (F, F) {
    let t8670 = t739 * t8669;
    let t8671 = t8670 * t590;
    let t8675 = t1890 * t8669 * t590;
    let t8679 = t5241 * t8502 * t590;
    let t8682 = t739 * t8502;
    let t8683 = t8682 * t590;
    let t8688 = -F::new(0.18404604457881959845e2) * t2087 * t8629 + F::new(0.71500979903700853338e0) * t780 * t8634 - F::new(0.21450293971110256002e1) * t8638 * t2639 - F::new(0.10725146985555128001e1) * t3025 * t7736 - F::new(0.92023022289409799224e1) * t2194 * t3019 - F::new(0.92023022289409799224e1) * t813 * t8646 - F::new(0.46011511144704899612e1) * t813 * t8650 + F::new(0.46011511144704899612e1) * t2178 * t3028 - F::new(0.61348681526273199482e1) * t1998 * t8655 + F::new(0.47667319935800568892e0) * t2004 * t8658 + F::new(0.35750489951850426669e0) * t2061 * t3040 - F::new(0.71500979903700853338e0) * t8663 * t2009 - F::new(0.71500979903700853338e0) * t8666 * t2009 - F::new(0.1022478025437886658e1) * t1986 * t8671 + F::new(0.1022478025437886658e1) * t5840 * t8675 + F::new(0.30674340763136599742e1) * t5640 * t8679 + F::new(0.2044956050875773316e1) * t1991 * t8683 + F::new(0.47667319935800568892e0) * t3046 * t784;
    (t8682, t8688)
}
