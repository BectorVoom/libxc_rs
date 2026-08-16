//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 733/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk733(t1564: f64, t911: f64, t6907: f64, t6914: f64, t4585: f64, t888: f64, t4598: f64, t907: f64, t1429: f64, t1580: f64, t1641: f64, t2375: f64, t2421: f64, t2428: f64, t2437: f64, t2441: f64, t2452: f64, t4379: f64, t4634: f64, t4637: f64, t541: f64, t557: f64, t574: f64, t597: f64, t6876: f64, t6881: f64, t6889: f64, t6897: f64, t6900: f64, t6904: f64, t6909: f64, t6912: f64, t908: f64, t918: f64) -> (f64, f64) {
    let t6915 = t911 * t1564;
    let t6916 = t6915 * t6907;
    let t6917 = t6914 * t6916;
    let t6919 = t4585 * t888;
    let t6922 = t4598 * t907;
    let t6931 = 0.1022478025437886658e1_f64 * t597 * t6876 - 0.61348681526273199482e1_f64 * t1641 * t2452 + 0.61348681526273199482e1_f64 * t597 * t6881 - 0.23005755572352449806e1_f64 * t4634 * t908 - 0.46011511144704899612e1_f64 * t1641 * t2428 - 0.23005755572352449806e1_f64 * t574 * t6889 + 0.23005755572352449806e1_f64 * t4637 * t918 - 0.19171462976960374838e0_f64 * t6897 - 0.25561950635947166452e0_f64 * t6900 + 0.79445533226334281486e-1_f64 * t4379 * t2375 + 0.79445533226334281486e-1_f64 * t1429 * t6904 - 0.38342925953920749676e0_f64 * t6909 + 0.9585731488480187419e0_f64 * t6912 - 0.57514388930881124514e0_f64 * t6917 + 0.79445533226334281487e-1_f64 * t557 * t6919 - 0.1022478025437886658e1_f64 * t574 * t6922 + 0.61348681526273199482e1_f64 * t1580 * t2421 + 0.47667319935800568892e0_f64 * t2437 * t541 + 0.47667319935800568892e0_f64 * t2441 * t541;
    (t6917, t6931)
}
