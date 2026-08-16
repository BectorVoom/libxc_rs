//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2597/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2597(t11692: f64, t11697: f64, t15563: f64, t15743: f64, t3490: f64, t15239: f64, t486: f64, t11698: f64, t15569: f64, t15498: f64, t3523: f64, t11734: f64, t1174: f64, t11774: f64, t11863: f64, t1216: f64, t15637: f64, t3440: f64, t3515: f64, t44932: f64, t4582: f64, t4984: f64, t5005: f64, t5024: f64, t50857: f64, t50861: f64) -> (f64, f64) {
    let t52649 = t11692 * t11697 * t15563;
    let t52653 = t3490 * t15743;
    let t52659 = t486 * t15239;
    let t52664 = t15569 * t11698;
    let t52666 = t15498 * t3523;
    let t52668 = -5.0_f64 / 864.0_f64 * t5024 * t11774 + t1174 * t3440 * t50857 / 216.0_f64 + t1174 * t3440 * t50861 / 6.0_f64 + t52649 / 2304.0_f64 - t5005 * t11863 / 768.0_f64 + 5.0_f64 / 3456.0_f64 * t52653 - t44932 * t4984 / 1024.0_f64 - t11734 * t15637 / 512.0_f64 - t3515 * t4582 * t52659 * t1216 / 1024.0_f64 + t52664 / 216.0_f64 + t52666 / 216.0_f64;
    (t52659, t52668)
}
