//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 805/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk805<F: Float>(t2586: F, t590: F, t1966: F, t1986: F, t1991: F, t2009: F, t2043: F, t2103: F, t2201: F, t2621: F, t2638: F, t2649: F, t2654: F, t5629: F, t5640: F, t5669: F, t5715: F, t5840: F, t5983: F, t6096: F, t7630: F, t7631: F, t7635: F, t7638: F, t7644: F, t7647: F, t7650: F, t7653: F, t7656: F, t7660: F, t7664: F, t7668: F, t7672: F, t7676: F, t7679: F, t7682: F, t807: F, t813: F) -> F {
    let t7689 = t2586 * t590;
    let t7692 = -F::cast_from(0.14300195980740170668e1_f64) * t7630 * t7631 - F::cast_from(0.71500979903700853338e0_f64) * t7635 * t2009 + F::cast_from(0.71500979903700853338e0_f64) * t7638 * t6096 - F::cast_from(0.47667319935800568892e0_f64) * t2649 * t5715 + F::cast_from(0.46011511144704899612e1_f64) * t5629 * t7644 + F::cast_from(0.61348681526273199482e1_f64) * t807 * t7647 - F::cast_from(0.46011511144704899612e1_f64) * t2201 * t7650 - F::cast_from(0.14300195980740170668e1_f64) * t2638 * t7653 - F::cast_from(0.12269736305254639896e2_f64) * t813 * t7656 - F::cast_from(0.51123901271894332902e1_f64) * t1966 * t7660 + F::cast_from(0.30674340763136599742e1_f64) * t5640 * t7664 + F::cast_from(0.2044956050875773316e1_f64) * t1991 * t7668 - F::cast_from(0.1022478025437886658e1_f64) * t1986 * t7672 + F::cast_from(0.1022478025437886658e1_f64) * t5840 * t7676 + F::cast_from(0.95334639871601137784e0_f64) * t2103 * t7679 - F::cast_from(0.71500979903700853338e0_f64) * t5983 * t7682 + F::cast_from(0.35750489951850426669e0_f64) * t2043 * t2654 + F::cast_from(0.1022478025437886658e1_f64) * t5669 * t2621 + F::cast_from(0.1022478025437886658e1_f64) * t1991 * t7689;
    t7692
}
