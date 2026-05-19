//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1211/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1211<F: Float>(t22015: F, t25001: F, t894: F, t7379: F, t7382: F, t888: F, t7386: F, t7388: F, t24985: F, t2748: F, t1: F, t10825: F, t23503: F, t23595: F, t23600: F, t23658: F, t24567: F, t24636: F, t24644: F, t24646: F, t24652: F, t24654: F, t24660: F, t24978: F, t24986: F, t24989: F, t24995: F, t2598: F, t2609: F, t2672: F, t2722: F, t285: F, t289: F, t297: F, t313: F, t3608: F, t7383: F, t7389: F, t7421: F, t7886: F, t862: F, t874: F, t893: F, sigma0: F) -> (F, F) {
    let t25003 = t894 * t25001 * t22015;
    let t25007 = t7379 * t888 * t7382;
    let t25010 = t7386 * t888 * t7388;
    let t25012 = t2748 * t24985;
    let t25017 = -t862 * t2722 * t23658 / F::new(36.0) + t862 * t3608 * t23595 / F::new(54.0) + F::cast_from(0.50489339006693751717e0_f64) * t24636 + F::new(7.0) / F::new(108.0) * t862 * t10825 * t23600 + F::cast_from(0.71947308084538596198e1_f64) * t7421 * t2609 - F::cast_from(0.47333755318775392234e-1_f64) * t24644 + F::cast_from(0.95929744112718128262e1_f64) * t24646 + F::new(1309.0) / F::new(486.0) * sigma0 * t23503 * t285 * t289 - F::new(154.0) / F::new(243.0) * t24652 - F::cast_from(0.15146801702008125515e1_f64) * t24654 + F::cast_from(0.2951381987273961e-1_f64) * t893 * t24660 + F::cast_from(0.35500316489081544176e-1_f64) * t874 * t313 * t24978 * t1 * t297 + F::cast_from(0.22676282118978851028e6_f64) * t24986 * t7389 + F::cast_from(0.3283935570557285894e5_f64) * t24989 * t313 * t24567 * t2672 * t1 - F::cast_from(0.23456682646837756387e4_f64) * t24995 * t313 * t24567 * t1 * t297 - F::cast_from(0.96590683219875087274e-1_f64) * t893 * t25003 + F::cast_from(0.28345352648723563784e5_f64) * t25007 - F::cast_from(0.28345352648723563785e5_f64) * t25010 - F::cast_from(0.22676282118978851027e6_f64) * t25012 * t7383 + F::cast_from(0.61174099372587555274e0_f64) * t7886 * t2598;
    (t25003, t25017)
}
