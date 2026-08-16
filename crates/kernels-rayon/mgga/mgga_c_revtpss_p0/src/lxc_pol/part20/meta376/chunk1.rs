//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1364/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1364(t10665: f64, t125: f64, t10111: f64, t849: f64, t9720: f64, t685: f64, t775: f64, t855: f64, t10489: f64, t10770: f64, t10771: f64, t2477: f64, t2646: f64, t2745: f64, t2747: f64, t2749: f64, t40251: f64, t40393: f64, t40395: f64, t40399: f64, t40403: f64, t40409: f64, t40411: f64, t40413: f64, t40421: f64, t40425: f64, t40429: f64, t40438: f64, t40440: f64, t825: f64, t827: f64, t828: f64, t851: f64) -> (f64, f64) {
    let t40446 = t125 * t10665;
    let t40452 = t10111 * t849 * t9720;
    let t40455 = t40452 * t855 * t685 * t775;
    let t40457 = -0.34013387707001991332e-1_f64 * t40393 - 0.34013387707001991332e-1_f64 * t40395 + 0.68026775414003982664e-1_f64 * t40399 + 0.30492001685571196935e-3_f64 * t40403 - 0.80328230880474379775e-6_f64 * t40409 + 0.36585828794086175548e-2_f64 * t40411 + 0.40015750243531754508e-2_f64 * t40413 - 0.21437009059034868486e-3_f64 * t825 * t827 * t828 * t40251 + 0.30492001685571196936e-2_f64 * t40421 - 0.51384669507166276316e-2_f64 * t40425 + 0.85748036236139473944e-4_f64 * t40429 + 0.17149607247227894789e-1_f64 * t851 * t2477 * t828 * t10489 * t775 - 0.15246000842785598467e-3_f64 * t40438 - 0.48018900292238105408e-1_f64 * t40440 - 0.25724410870841842184e-1_f64 * t2745 * t10770 * t10771 * t2646 + 0.34299214494455789577e-2_f64 * t2745 * t2747 * t40446 * t2749 - 0.32131292352189751911e-5_f64 * t40455;
    (t40446, t40457)
}
