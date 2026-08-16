//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2335/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2335(t27834: f64, t3640: f64, t11947: f64, t8090: f64, t1254: f64, t1256: f64, t15834: f64, t1763: f64, t193: f64, t24905: f64, t24909: f64, t27838: f64, t27843: f64, t336: f64, t3633: f64, t3637: f64, t4700: f64, t5091: f64, t64447: f64, t7398: f64, t86513: f64, t86517: f64, t86524: f64, t94341: f64, t94385: f64, t94428: f64, t94464: f64, t94498: f64, t94530: f64, t94564: f64, t94605: f64, t94637: f64, t94673: f64, t94698: f64, t94734: f64, t94770: f64, t95844: f64, t95876: f64, t95913: f64) -> f64 {
    let t95921 = t27834 * t3640;
    let t95925 = t8090 * t11947;
    let t95952 = t193 * t336 * (t94341 + t94385 + t94428 + t94464 + t94498 + t94530 + t94564 + t94605 + t94637 + t94673 + t94698 + t94734 + t94770 + t95844 + t95876 + t95913) * t1256 - 2.0_f64 * t4700 * t95921 * t1254 + 2.0_f64 * t4700 * t95925 * t3637 - t4700 * t27838 * t3633 - t4700 * t86513 * t1763 + 4.0_f64 * t4700 * t86517 * t27843 - 2.0_f64 * t4700 * t24905 * t5091 - 6.0_f64 * t4700 * t86524 * t1763 * t3637 + 4.0_f64 * t4700 * t24909 * t64447 + 2.0_f64 * t4700 * t24909 * t1763 * t3633 - t4700 * t7398 * t15834;
    t95952
}
