//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1182/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1182<F: Float>(t25093: F, t55901: F, t894: F, t2596: F, t55906: F, t25001: F, t4776: F, t8201: F, t1: F, t14300: F, t16975: F, t2640: F, t2722: F, t297: F, t313: F, t40391: F, t41757: F, t41832: F, t51102: F, t51164: F, t51169: F, t56740: F, t57530: F, t7449: F, t7491: F, t862: F, t874: F, t893: F) -> (F, F, F, F, F) {
    let t57537 = t894 * t25093 * t55901;
    let t57541 = t894 * t2596 * t55906;
    let t57545 = t894 * t25001 * t55901;
    let t57554 = t8201 * t4776;
    let t57561 = 0.23666877659387696117e0 * t2640 * t40391 * t16975 + 0.48295341609937543636e-2 * t41757 + 0.35500316489081544176e-1 * t874 * t313 * t57530 * t1 * t297 + 0.10866451862235947318e0 * t893 * t57537 + 0.90553765518632894319e-2 * t893 * t57541 - 0.96590683219875087274e-1 * t893 * t57545 + 0.18933502127510156893e0 * t51164 + 0.18933502127510156893e0 * t51169 - t41832 / 162.0 + t862 * t2722 * t56740 / 8.0 + 0.36629113921839320676e2 * t7491 * t14300 * t57554 - 0.18314556960919660338e2 * t7449 * t14300 * t51102;
    (t57537, t57541, t57545, t57554, t57561)
}
