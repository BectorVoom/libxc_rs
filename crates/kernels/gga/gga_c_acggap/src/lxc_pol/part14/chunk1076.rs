//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1076/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1076<F: Float>(t315: F, t323: F, t9767: F, t2138: F, t2147: F, t322: F, t9789: F, t8419: F, t8998: F, t1620: F, t2146: F, t2338: F, t30023: F, t32187: F, t32196: F, t32210: F, t36477: F, t36482: F, t36498: F, t36504: F, t463: F, t8415: F, t9003: F, t9010: F, t9015: F, t9044: F, t9497: F) -> (F,) {
    let t40884 = t315 * t9767 * t323;
    let t40895 = t2138 * t2147 * t9789 * t322;
    let t40905 = t8998 * t8419;
    let t40907 = -0.69389025505641595696e1 * t36477 + t36482 - 0.65854491829355115987e0 * t40884 + 0.17347256376410398924e1 * t32187 + 0.17347256376410398924e1 * t9003 * t8415 + 0.26341796731742046394e1 * t9010 * t1620 + 0.8673628188205199462e0 * t9003 * t9015 - 0.34694512752820797848e1 * t40895 - t36498 + 0.8673628188205199462e0 * t32196 - 0.13170898365871023197e1 * t36504 - 0.8673628188205199462e0 * t2338 * t9044 - t32210 + 0.10408353825846239354e2 * t2146 * t30023 * t9497 * t463 + 0.17347256376410398924e1 * t40905;
    (t40907,)
}
