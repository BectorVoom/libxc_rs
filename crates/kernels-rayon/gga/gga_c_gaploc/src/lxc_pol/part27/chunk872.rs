//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 872/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk872(t739: f64, t8669: f64, t590: f64, t1890: f64, t5241: f64, t8502: f64, t1986: f64, t1991: f64, t1998: f64, t2004: f64, t2009: f64, t2061: f64, t2087: f64, t2178: f64, t2194: f64, t2639: f64, t3019: f64, t3025: f64, t3028: f64, t3040: f64, t3046: f64, t5640: f64, t5840: f64, t7736: f64, t780: f64, t784: f64, t813: f64, t8629: f64, t8634: f64, t8638: f64, t8646: f64, t8650: f64, t8655: f64, t8658: f64, t8663: f64, t8666: f64) -> (f64, f64) {
    let t8670 = t739 * t8669;
    let t8671 = t8670 * t590;
    let t8675 = t1890 * t8669 * t590;
    let t8679 = t5241 * t8502 * t590;
    let t8682 = t739 * t8502;
    let t8683 = t8682 * t590;
    let t8688 = -0.18404604457881959845e2_f64 * t2087 * t8629 + 0.71500979903700853338e0_f64 * t780 * t8634 - 0.21450293971110256002e1_f64 * t8638 * t2639 - 0.10725146985555128001e1_f64 * t3025 * t7736 - 0.92023022289409799224e1_f64 * t2194 * t3019 - 0.92023022289409799224e1_f64 * t813 * t8646 - 0.46011511144704899612e1_f64 * t813 * t8650 + 0.46011511144704899612e1_f64 * t2178 * t3028 - 0.61348681526273199482e1_f64 * t1998 * t8655 + 0.47667319935800568892e0_f64 * t2004 * t8658 + 0.35750489951850426669e0_f64 * t2061 * t3040 - 0.71500979903700853338e0_f64 * t8663 * t2009 - 0.71500979903700853338e0_f64 * t8666 * t2009 - 0.1022478025437886658e1_f64 * t1986 * t8671 + 0.1022478025437886658e1_f64 * t5840 * t8675 + 0.30674340763136599742e1_f64 * t5640 * t8679 + 0.2044956050875773316e1_f64 * t1991 * t8683 + 0.47667319935800568892e0_f64 * t3046 * t784;
    (t8682, t8688)
}
