//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2217/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2217(t25655: f64, t82895: f64, t25661: f64, t1036: f64, t25664: f64, t1611: f64, t23528: f64, t23436: f64, t4640: f64, t14507: f64, t23536: f64, t1025: f64, t1046: f64, t1622: f64, t23504: f64, t25580: f64, t25683: f64, t3057: f64, t3134: f64, t378: f64, t4616: f64, t6758: f64, t82868: f64, t83080: f64, t83082: f64, t83098: f64) -> f64 {
    let t88575 = 0.40372756094140390856e-3_f64 * t82895 * t25655;
    let t88577 = 0.20186378047070195428e-3_f64 * t82895 * t25661;
    let t88582 = t25664 * t1036 / 1152.0_f64;
    let t88584 = t1611 * t23528;
    let t88591 = t4640 * t23436;
    let t88594 = t14507 * t23536;
    let t88597 = t83080 - t83082 / 216.0_f64 + 0.10093189023535097714e-3_f64 * t25683 * t23504 + t88575 - t88577 - t4616 * t6758 * t378 / 144.0_f64 + t88582 + 11.0_f64 / 324.0_f64 * t83098 - t88584 * t1046 / 216.0_f64 + 19.0_f64 / 1296.0_f64 * t82868 * t1622 + t25580 * t3057 / 2304.0_f64 - t88591 * t1025 / 144.0_f64 + t88594 * t3134 / 768.0_f64;
    t88597
}
