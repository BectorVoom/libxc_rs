//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 519/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk519<F: Float>(t1966: F, t1991: F, t1998: F, t2004: F, t2009: F, t2103: F, t2598: F, t2601: F, t2605: F, t2608: F, t2613: F, t2619: F, t2621: F, t2625: F, t2629: F, t2631: F, t2635: F, t2638: F, t2639: F, t2642: F, t2646: F, t2649: F, t2654: F, t2658: F, t2660: F, t780: F, t807: F, t813: F, t833: F) -> F {
    let t2663 = F::cast_from(0.30674340763136599741e1_f64) * t833 * t2598 + F::cast_from(0.14896037479937677779e-1_f64) * t2601 - F::cast_from(0.14896037479937677779e-1_f64) * t2605 + F::cast_from(0.12780975317973583226e0_f64) * t2608 - F::cast_from(0.14896037479937677779e-1_f64) * t2613 + F::cast_from(0.95857314884801874192e-1_f64) * t2619 + F::cast_from(0.51123901271894332902e0_f64) * t1991 * t2621 - F::cast_from(0.51123901271894332902e0_f64) * t1966 * t2625 - F::cast_from(0.29792074959875355558e-1_f64) * t2629 + F::cast_from(0.71500979903700853338e0_f64) * t2103 * t2631 - F::cast_from(0.46011511144704899612e1_f64) * t813 * t2635 - F::cast_from(0.10725146985555128001e1_f64) * t2638 * t2639 + F::cast_from(0.23005755572352449806e1_f64) * t807 * t2642 - F::cast_from(0.23005755572352449806e1_f64) * t1998 * t2646 - F::cast_from(0.35750489951850426669e0_f64) * t2649 * t2009 + F::cast_from(0.35750489951850426669e0_f64) * t780 * t2654 + F::cast_from(0.14896037479937677779e-1_f64) * t2658 + F::cast_from(0.35750489951850426669e0_f64) * t2004 * t2660;
    t2663
}
