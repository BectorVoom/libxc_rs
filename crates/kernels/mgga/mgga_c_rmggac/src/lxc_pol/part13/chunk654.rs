//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 654/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk654<F: Float>(t352: F, t9551: F, t321: F, t9540: F, t333: F, t338: F, t9486: F, t118: F, t326: F, t4669: F, t5148: F, t5155: F, t5266: F, t7826: F, t7832: F, t7842: F, t8242: F, t8243: F, t8940: F, t8944: F, t8966: F, t9302: F) -> (F, F) {
    let t9552 = t9551 * t352;
    let t9555 = t9540 * t321;
    let t9558 = t9540 * t333;
    let t9565 = t338 * t9486;
    let t9566 = t118 * t9565;
    let t9568 = t9551 * t321;
    let t9571 = t9551 * t333;
    let t9574 = t8242 - t8243 - 0.2993560425465952141e-1 * t8944 - 0.59871208509319042821e-1 * t326 * t9302 + 0.11974241701863808564e0 * t8940 * t9552 - 0.17961362552795712846e0 * t4669 * t9555 + 0.23948483403727617128e0 * t5155 * t9558 - 0.8980681276397856423e-1 * t8966 + 0.54549323308490683461e-1 * t7826 - 0.72732431077987577948e-1 * t7832 - 0.18183107769496894487e-1 * t7842 + 0.19957069503106347607e-1 * t9566 - 0.11974241701863808564e0 * t5148 * t9568 + 0.11974241701863808564e0 * t5266 * t9571;
    (t9565, t9574)
}
