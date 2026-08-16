//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1168/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1168(t10405: f64, t10408: f64, t10411: f64, t10454: f64, t12227: f64, t12431: f64, t1413: f64, t19467: f64, t23711: f64, t2507: f64, t2513: f64, t2528: f64, t28684: f64, t28721: f64, t3337: f64, t3340: f64, t3356: f64, t4218: f64, t448: f64, t453: f64, t459: f64, t4772: f64, t4828: f64, t6634: f64, t8599: f64, t8607: f64, t8661: f64, t995: f64) -> f64 {
    let t28759 = -0.165625e-1_f64 * t453 * (t28684 + t28721) + 0.298125e0_f64 * t4772 * t10454 * t448 - 0.59625e0_f64 * t4828 * t10454 * t459 + 0.496875e-1_f64 * t12431 * t3337 + 0.496875e-1_f64 * t4218 * t8661 - 0.19875e0_f64 * t12227 * t8607 + 0.298125e0_f64 * t23711 * t2513 + 0.298125e0_f64 * t19467 * t10405 + 0.298125e0_f64 * t4772 * t2507 * t3340 - 0.99375e-1_f64 * t6634 * t10408 - 0.99375e-1_f64 * t1413 * t8661 * t995 - 0.99375e-1_f64 * t1413 * t3337 * t2528 - 0.99375e-1_f64 * t6634 * t10411 - 0.99375e-1_f64 * t8599 * t2528 - 0.99375e-1_f64 * t1413 * t2507 * t3356;
    t28759
}
