//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 612/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk612<F: Float>(t2862: F, t932: F, t2764: F, t2822: F, t2766: F, t2773: F, t2778: F, t2782: F, t2800: F, t2808: F, t2816: F, t2818: F, t2824: F, t2828: F, t2831: F, t2834: F) -> (F, F, F, F) {
    let t2863 = t2862 * t932;
    let t2868 = F::cast_from(0.68863333333333333333e0_f64) * t2764;
    let t2875 = F::cast_from(0.17365833333333333333e0_f64) * t2822;
    let t2880 = -F::cast_from(0.17648625e1_f64) * t2800 + F::cast_from(0.3529725e1_f64) * t2808 + t2868 + F::cast_from(0.34431666666666666666e0_f64) * t2766 - F::cast_from(0.34431666666666666667e0_f64) * t2773 + F::cast_from(0.103295e1_f64) * t2778 - F::cast_from(0.516475e0_f64) * t2782 + F::cast_from(0.31558125e0_f64) * t2816 + F::cast_from(0.6311625e0_f64) * t2818 + t2875 + F::cast_from(0.13892666666666666667e0_f64) * t2824 - F::cast_from(0.34731666666666666667e-1_f64) * t2828 + F::cast_from(0.20839e0_f64) * t2831 - F::cast_from(0.104195e0_f64) * t2834;
    (t2863, t2868, t2875, t2880)
}
