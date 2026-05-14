//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 758/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk758<F: Float>(t5241: F, t7291: F, t590: F, t739: F, t7068: F, t1890: F, t2582: F, t4673: F, t1457: F, t7132: F, t2586: F, t1966: F, t1986: F, t1991: F, t2009: F, t2043: F, t2103: F, t2201: F, t2621: F, t2638: F, t2649: F, t2654: F, t5629: F, t5640: F, t5669: F, t5715: F, t5840: F, t5983: F, t6096: F, t7630: F, t7631: F, t7635: F, t7638: F, t7644: F, t7647: F, t7650: F, t7653: F, t7656: F, t7660: F, t807: F, t813: F) -> (F, F, F, F) {
    let t7663 = t5241 * t7291;
    let t7664 = t7663 * t590;
    let t7667 = t739 * t7291;
    let t7668 = t7667 * t590;
    let t7671 = t739 * t7068;
    let t7672 = t7671 * t590;
    let t7675 = t1890 * t7068;
    let t7676 = t7675 * t590;
    let t7679 = t4673 * t2582;
    let t7682 = t1457 * t7132;
    let t7689 = t2586 * t590;
    let t7692 = -0.14300195980740170668e1 * t7630 * t7631 - 0.71500979903700853338e0 * t7635 * t2009 + 0.71500979903700853338e0 * t7638 * t6096 - 0.47667319935800568892e0 * t2649 * t5715 + 0.46011511144704899612e1 * t5629 * t7644 + 0.61348681526273199482e1 * t807 * t7647 - 0.46011511144704899612e1 * t2201 * t7650 - 0.14300195980740170668e1 * t2638 * t7653 - 0.12269736305254639896e2 * t813 * t7656 - 0.51123901271894332902e1 * t1966 * t7660 + 0.30674340763136599742e1 * t5640 * t7664 + 0.2044956050875773316e1 * t1991 * t7668 - 0.1022478025437886658e1 * t1986 * t7672 + 0.1022478025437886658e1 * t5840 * t7676 + 0.95334639871601137784e0 * t2103 * t7679 - 0.71500979903700853338e0 * t5983 * t7682 + 0.35750489951850426669e0 * t2043 * t2654 + 0.1022478025437886658e1 * t5669 * t2621 + 0.1022478025437886658e1 * t1991 * t7689;
    (t7667, t7671, t7682, t7692)
}
