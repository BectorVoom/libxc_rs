//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 786/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk786(t2764: f64, t2822: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t2800: f64, t2808: f64, t2816: f64, t2818: f64, t2824: f64, t2828: f64, t2831: f64, t2834: f64) -> (f64, f64, f64) {
    let t2912 = 0.40256666666666666667e0_f64 * t2764;
    let t2919 = 0.137975e0_f64 * t2822;
    let t2924 = -0.1294625e1_f64 * t2800 + 0.258925e1_f64 * t2808 + t2912 + 0.20128333333333333334e0_f64 * t2766 - 0.20128333333333333333e0_f64 * t2773 + 0.60385e0_f64 * t2778 - 0.301925e0_f64 * t2782 + 0.82524375e-1_f64 * t2816 + 0.16504875e0_f64 * t2818 + t2919 + 0.11038e0_f64 * t2824 - 0.27595e-1_f64 * t2828 + 0.16557e0_f64 * t2831 - 0.82785e-1_f64 * t2834;
    (t2912, t2919, t2924)
}
