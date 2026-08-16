//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 543/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk543<F: Float>(t2906: F, t951: F, t2764: F, t2822: F, t2766: F, t2773: F, t2778: F, t2782: F, t2800: F, t2808: F, t2816: F, t2818: F, t2824: F, t2828: F, t2831: F, t2834: F) -> (F, F) {
    let t2907 = t2906 * t951;
    let t2912 = F::cast_from(0.40256666666666666667e0_f64) * t2764;
    let t2919 = F::cast_from(0.137975e0_f64) * t2822;
    let t2924 = -F::cast_from(0.1294625e1_f64) * t2800 + F::cast_from(0.258925e1_f64) * t2808 + t2912 + F::cast_from(0.20128333333333333334e0_f64) * t2766 - F::cast_from(0.20128333333333333333e0_f64) * t2773 + F::cast_from(0.60385e0_f64) * t2778 - F::cast_from(0.301925e0_f64) * t2782 + F::cast_from(0.82524375e-1_f64) * t2816 + F::cast_from(0.16504875e0_f64) * t2818 + t2919 + F::cast_from(0.11038e0_f64) * t2824 - F::cast_from(0.27595e-1_f64) * t2828 + F::cast_from(0.16557e0_f64) * t2831 - F::cast_from(0.82785e-1_f64) * t2834;
    (t2907, t2924)
}
