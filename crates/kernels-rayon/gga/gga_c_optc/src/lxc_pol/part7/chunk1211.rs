//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1211/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1211(t22015: f64, t25001: f64, t894: f64, t7379: f64, t7382: f64, t888: f64, t7386: f64, t7388: f64, t24985: f64, t2748: f64, t1: f64, t10825: f64, t23503: f64, t23595: f64, t23600: f64, t23658: f64, t24567: f64, t24636: f64, t24644: f64, t24646: f64, t24652: f64, t24654: f64, t24660: f64, t24978: f64, t24986: f64, t24989: f64, t24995: f64, t2598: f64, t2609: f64, t2672: f64, t2722: f64, t285: f64, t289: f64, t297: f64, t313: f64, t3608: f64, t7383: f64, t7389: f64, t7421: f64, t7886: f64, t862: f64, t874: f64, t893: f64, sigma0: f64) -> (f64, f64) {
    let t25003 = t894 * t25001 * t22015;
    let t25007 = t7379 * t888 * t7382;
    let t25010 = t7386 * t888 * t7388;
    let t25012 = t2748 * t24985;
    let t25017 = -t862 * t2722 * t23658 / 36.0_f64 + t862 * t3608 * t23595 / 54.0_f64 + 0.50489339006693751717e0_f64 * t24636 + 7.0_f64 / 108.0_f64 * t862 * t10825 * t23600 + 0.71947308084538596198e1_f64 * t7421 * t2609 - 0.47333755318775392234e-1_f64 * t24644 + 0.95929744112718128262e1_f64 * t24646 + 1309.0_f64 / 486.0_f64 * sigma0 * t23503 * t285 * t289 - 154.0_f64 / 243.0_f64 * t24652 - 0.15146801702008125515e1_f64 * t24654 + 0.2951381987273961e-1_f64 * t893 * t24660 + 0.35500316489081544176e-1_f64 * t874 * t313 * t24978 * t1 * t297 + 0.22676282118978851028e6_f64 * t24986 * t7389 + 0.3283935570557285894e5_f64 * t24989 * t313 * t24567 * t2672 * t1 - 0.23456682646837756387e4_f64 * t24995 * t313 * t24567 * t1 * t297 - 0.96590683219875087274e-1_f64 * t893 * t25003 + 0.28345352648723563784e5_f64 * t25007 - 0.28345352648723563785e5_f64 * t25010 - 0.22676282118978851027e6_f64 * t25012 * t7383 + 0.61174099372587555274e0_f64 * t7886 * t2598;
    (t25003, t25017)
}
