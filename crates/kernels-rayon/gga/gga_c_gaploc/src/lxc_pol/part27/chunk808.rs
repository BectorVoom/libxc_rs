//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 808/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk808(t2586: f64, t590: f64, t1966: f64, t1986: f64, t1991: f64, t2009: f64, t2043: f64, t2103: f64, t2201: f64, t2621: f64, t2638: f64, t2649: f64, t2654: f64, t5629: f64, t5640: f64, t5669: f64, t5715: f64, t5840: f64, t5983: f64, t6096: f64, t7630: f64, t7631: f64, t7635: f64, t7638: f64, t7644: f64, t7647: f64, t7650: f64, t7653: f64, t7656: f64, t7660: f64, t7664: f64, t7668: f64, t7672: f64, t7676: f64, t7679: f64, t7682: f64, t807: f64, t813: f64) -> f64 {
    let t7689 = t2586 * t590;
    let t7692 = -0.14300195980740170668e1_f64 * t7630 * t7631 - 0.71500979903700853338e0_f64 * t7635 * t2009 + 0.71500979903700853338e0_f64 * t7638 * t6096 - 0.47667319935800568892e0_f64 * t2649 * t5715 + 0.46011511144704899612e1_f64 * t5629 * t7644 + 0.61348681526273199482e1_f64 * t807 * t7647 - 0.46011511144704899612e1_f64 * t2201 * t7650 - 0.14300195980740170668e1_f64 * t2638 * t7653 - 0.12269736305254639896e2_f64 * t813 * t7656 - 0.51123901271894332902e1_f64 * t1966 * t7660 + 0.30674340763136599742e1_f64 * t5640 * t7664 + 0.2044956050875773316e1_f64 * t1991 * t7668 - 0.1022478025437886658e1_f64 * t1986 * t7672 + 0.1022478025437886658e1_f64 * t5840 * t7676 + 0.95334639871601137784e0_f64 * t2103 * t7679 - 0.71500979903700853338e0_f64 * t5983 * t7682 + 0.35750489951850426669e0_f64 * t2043 * t2654 + 0.1022478025437886658e1_f64 * t5669 * t2621 + 0.1022478025437886658e1_f64 * t1991 * t7689;
    t7692
}
