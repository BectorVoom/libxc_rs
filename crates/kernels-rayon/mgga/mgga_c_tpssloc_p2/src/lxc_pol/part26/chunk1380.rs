//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1380/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1380(t265: f64, t504: f64, t24901: f64, t3640: f64, t11947: f64, t7394: f64, t2157: f64, t43706: f64, t11940: f64, t11944: f64, t1254: f64, t1256: f64, t193: f64, t24905: f64, t24909: f64, t336: f64, t3633: f64, t3637: f64, t4700: f64, t51906: f64, t7398: f64, t83543: f64, t85673: f64, t85713: f64, t85749: f64, t85791: f64, t86399: f64, t86436: f64, t86468: f64, t86506: f64) -> f64 {
    let t505 = t265 < t504;
    let t86513 = t24901 * t3640;
    let t86517 = t7394 * t11947;
    let t86524 = t2157 * t43706;
    let t86534 = piecewise3(t505, t193 * t336 * (t85673 + t85713 + t85749 + t85791 + t86399 + t86436 + t86468 + t86506) * t1256 - 3.0_f64 * t4700 * t86513 * t1254 + 6.0_f64 * t4700 * t86517 * t3637 - 3.0_f64 * t4700 * t24905 * t3633 - 6.0_f64 * t4700 * t86524 * t11944 + 6.0_f64 * t4700 * t24909 * t51906 - t4700 * t7398 * t11940, t83543);
    t86534
}
