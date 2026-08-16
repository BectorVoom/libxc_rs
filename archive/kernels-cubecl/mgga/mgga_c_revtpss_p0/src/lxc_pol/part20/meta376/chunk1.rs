//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1364/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1364<F: Float>(t10665: F, t125: F, t10111: F, t849: F, t9720: F, t685: F, t775: F, t855: F, t10489: F, t10770: F, t10771: F, t2477: F, t2646: F, t2745: F, t2747: F, t2749: F, t40251: F, t40393: F, t40395: F, t40399: F, t40403: F, t40409: F, t40411: F, t40413: F, t40421: F, t40425: F, t40429: F, t40438: F, t40440: F, t825: F, t827: F, t828: F, t851: F) -> (F, F) {
    let t40446 = t125 * t10665;
    let t40452 = t10111 * t849 * t9720;
    let t40455 = t40452 * t855 * t685 * t775;
    let t40457 = -F::cast_from(0.34013387707001991332e-1_f64) * t40393 - F::cast_from(0.34013387707001991332e-1_f64) * t40395 + F::cast_from(0.68026775414003982664e-1_f64) * t40399 + F::cast_from(0.30492001685571196935e-3_f64) * t40403 - F::cast_from(0.80328230880474379775e-6_f64) * t40409 + F::cast_from(0.36585828794086175548e-2_f64) * t40411 + F::cast_from(0.40015750243531754508e-2_f64) * t40413 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t827 * t828 * t40251 + F::cast_from(0.30492001685571196936e-2_f64) * t40421 - F::cast_from(0.51384669507166276316e-2_f64) * t40425 + F::cast_from(0.85748036236139473944e-4_f64) * t40429 + F::cast_from(0.17149607247227894789e-1_f64) * t851 * t2477 * t828 * t10489 * t775 - F::cast_from(0.15246000842785598467e-3_f64) * t40438 - F::cast_from(0.48018900292238105408e-1_f64) * t40440 - F::cast_from(0.25724410870841842184e-1_f64) * t2745 * t10770 * t10771 * t2646 + F::cast_from(0.34299214494455789577e-2_f64) * t2745 * t2747 * t40446 * t2749 - F::cast_from(0.32131292352189751911e-5_f64) * t40455;
    (t40446, t40457)
}
