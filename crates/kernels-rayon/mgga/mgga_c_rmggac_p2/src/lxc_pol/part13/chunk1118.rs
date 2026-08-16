//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1118/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1118(t41549: f64, t321: f64, t333: f64, t41538: f64, t41540: f64, t41542: f64, t41544: f64, t41560: f64, t41562: f64, t44194: f64, t44232: f64, t4669: f64, t5155: f64, t5266: f64, t833: f64, t866: f64, t8940: f64, t9540: f64, t9551: f64) -> f64 {
    let t44362 = 0.3193131120497015617e0_f64 * t41549;
    let t44368 = 0.11974241701863808564e0_f64 * t8940 * t9551 * t866 + 0.14369090042236570277e1_f64 * t41538 + 0.35922725105591425692e0_f64 * t41540 - 0.71845450211182851384e0_f64 * t41542 + 0.35922725105591425692e0_f64 * t41544 + 0.11974241701863808564e0_f64 * t5266 * t9540 * t866 - 0.35922725105591425692e0_f64 * t4669 * t44232 * t321 - 0.35922725105591425692e0_f64 * t4669 * t44194 * t321 - 0.17961362552795712846e0_f64 * t4669 * t9540 * t833 - t44362 + 0.47896966807455234256e0_f64 * t5155 * t44232 * t333 + 0.5987120850931904282e-1_f64 * t41560 + 0.8980681276397856423e-1_f64 * t41562;
    t44368
}
