//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 980/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk980<F: Float>(t41549: F, t321: F, t333: F, t41538: F, t41540: F, t41542: F, t41544: F, t41560: F, t41562: F, t44194: F, t44232: F, t4669: F, t5155: F, t5266: F, t833: F, t866: F, t8940: F, t9540: F, t9551: F) -> (F,) {
    let t44362 = 0.3193131120497015617e0 * t41549;
    let t44368 = 0.11974241701863808564e0 * t8940 * t9551 * t866 + 0.14369090042236570277e1 * t41538 + 0.35922725105591425692e0 * t41540 - 0.71845450211182851384e0 * t41542 + 0.35922725105591425692e0 * t41544 + 0.11974241701863808564e0 * t5266 * t9540 * t866 - 0.35922725105591425692e0 * t4669 * t44232 * t321 - 0.35922725105591425692e0 * t4669 * t44194 * t321 - 0.17961362552795712846e0 * t4669 * t9540 * t833 - t44362 + 0.47896966807455234256e0 * t5155 * t44232 * t333 + 0.5987120850931904282e-1 * t41560 + 0.8980681276397856423e-1 * t41562;
    (t44368,)
}
