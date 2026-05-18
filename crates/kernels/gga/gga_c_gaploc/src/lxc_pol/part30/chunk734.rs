//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 734/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk734<F: Float>(t1564: F, t911: F, t6907: F, t6914: F, t4585: F, t888: F, t4598: F, t907: F, t1429: F, t1580: F, t1641: F, t2375: F, t2421: F, t2428: F, t2437: F, t2441: F, t2452: F, t4379: F, t4634: F, t4637: F, t541: F, t557: F, t574: F, t597: F, t6876: F, t6881: F, t6889: F, t6897: F, t6900: F, t6904: F, t6909: F, t6912: F, t908: F, t918: F) -> (F, F) {
    let t6915 = t911 * t1564;
    let t6916 = t6915 * t6907;
    let t6917 = t6914 * t6916;
    let t6919 = t4585 * t888;
    let t6922 = t4598 * t907;
    let t6931 = F::new(0.1022478025437886658e1) * t597 * t6876 - F::new(0.61348681526273199482e1) * t1641 * t2452 + F::new(0.61348681526273199482e1) * t597 * t6881 - F::new(0.23005755572352449806e1) * t4634 * t908 - F::new(0.46011511144704899612e1) * t1641 * t2428 - F::new(0.23005755572352449806e1) * t574 * t6889 + F::new(0.23005755572352449806e1) * t4637 * t918 - F::new(0.19171462976960374838e0) * t6897 - F::new(0.25561950635947166452e0) * t6900 + F::new(0.79445533226334281486e-1) * t4379 * t2375 + F::new(0.79445533226334281486e-1) * t1429 * t6904 - F::new(0.38342925953920749676e0) * t6909 + F::new(0.9585731488480187419e0) * t6912 - F::new(0.57514388930881124514e0) * t6917 + F::new(0.79445533226334281487e-1) * t557 * t6919 - F::new(0.1022478025437886658e1) * t574 * t6922 + F::new(0.61348681526273199482e1) * t1580 * t2421 + F::new(0.47667319935800568892e0) * t2437 * t541 + F::new(0.47667319935800568892e0) * t2441 * t541;
    (t6917, t6931)
}
