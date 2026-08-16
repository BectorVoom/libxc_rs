//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2638/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2638(t6218: f64, t6739: f64, t15032: f64, t1756: f64, t19154: f64, t19170: f64, t19189: f64, t19190: f64, t19203: f64, t22354: f64, t22389: f64, t3610: f64, t3624: f64, t470: f64, t493: f64, t494: f64, t5064: f64, t5069: f64, t5079: f64, t53592: f64, t6256: f64, t6261: f64, t65254: f64, t66787: f64, t73576: f64, t73592: f64) -> (f64, f64) {
    let t73755 = t6218 * t6739;
    let t73789 = -3.0_f64 * t19189 * t22354 * t3624 + 12.0_f64 * t19203 * t3610 * t6256 - 3.0_f64 * t22389 * t3624 * t5079 + t470 * t493 * t73592 + 3.0_f64 * t15032 * t6261 + 3.0_f64 * t1756 * t66787 + 3.0_f64 * t19154 * t53592 + 6.0_f64 * t19170 * t5064 + 3.0_f64 * t19190 * t5064 + t494 * t73576 + 6.0_f64 * t5069 * t65254;
    (t73755, t73789)
}
