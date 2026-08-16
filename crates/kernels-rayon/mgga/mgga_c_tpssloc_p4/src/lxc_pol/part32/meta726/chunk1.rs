//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2342/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2342(t29827: f64, t3640: f64, t103164: f64, t103213: f64, t103258: f64, t103279: f64, t103303: f64, t103341: f64, t103377: f64, t103415: f64, t103457: f64, t103488: f64, t104508: f64, t104534: f64, t104564: f64, t104596: f64, t104631: f64, t104669: f64, t1254: f64, t1256: f64, t1763: f64, t19262: f64, t193: f64, t24905: f64, t24909: f64, t27838: f64, t27843: f64, t336: f64, t4700: f64, t5091: f64, t6270: f64, t6274: f64, t7398: f64, t86517: f64, t86524: f64, t95921: f64, t95925: f64) -> f64 {
    let t104677 = t29827 * t3640;
    let t104708 = t193 * t336 * (t103164 + t103213 + t103258 + t103279 + t103303 + t103341 + t103377 + t103415 + t103457 + t103488 + t104508 + t104534 + t104564 + t104596 + t104631 + t104669) * t1256 - t4700 * t104677 * t1254 - 2.0_f64 * t4700 * t95921 * t1763 + 4.0_f64 * t4700 * t95925 * t27843 - 2.0_f64 * t4700 * t27838 * t5091 + 2.0_f64 * t4700 * t86517 * t6274 - 6.0_f64 * t4700 * t86524 * t6274 * t1254 + 4.0_f64 * t4700 * t24909 * t1763 * t5091 - t4700 * t24905 * t6270 + 2.0_f64 * t4700 * t24909 * t6270 * t1254 - t4700 * t7398 * t19262;
    t104708
}
