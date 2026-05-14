//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 592/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk592<F: Float>(t352: F, t8264: F, t2228: F, t321: F, t699: F, t848: F, t333: F, t118: F, t305: F, t326: F, t4669: F, t7845: F, t7847: F, t7849: F, t7853: F, t7856: F, t7863: F, t7865: F, t7867: F, t7869: F, t7877: F, t793: F, t8042: F, t8045: F, t8063: F, t8078: F, t8258: F, t8261: F, t838: F) -> (F, F, F, F, F) {
    let t8265 = t8264 * t352;
    let t8273 = t2228 * t321;
    let t8278 = t699 * t848;
    let t8281 = t2228 * t333;
    let t8290 = -0.40911992481368012596e-1 * t7845 - 0.20455996240684006298e-1 * t7847 + 0.2727466165424534173e-1 * t7849 + 0.68186654135613354325e-2 * t7853 - 0.2993560425465952141e-1 * t7856 - 0.39914139006212695214e-1 * t118 * t8258 - 0.35922725105591425692e0 * t4669 * t8261 - 0.79828278012425390428e-1 * t118 * t8265 + 0.11974241701863808564e0 * t793 * t8063 + 0.17961362552795712846e0 * t7863 - 0.35922725105591425692e0 * t7865 - 0.11974241701863808564e0 * t7867 + 0.11974241701863808564e0 * t305 * t8273 + 0.59871208509319042821e-1 * t305 * t8078 - 0.59871208509319042821e-1 * t326 * t8278 - 0.11974241701863808564e0 * t326 * t8281 + 0.23948483403727617128e0 * t838 * t8045 + 0.11974241701863808564e0 * t118 * t8042 + 0.35922725105591425692e0 * t7869 + 0.5987120850931904282e-1 * t7877;
    (t8265, t8273, t8278, t8281, t8290)
}
