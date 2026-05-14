//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 768/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk768<F: Float>(t40332: F, t40623: F, t41191: F, t41247: F, t41257: F, t41265: F, t41355: F, t41363: F, t41365: F, t40681: F, t41767: F, t42057: F, t40944: F, t40949: F, t40951: F, t38187: F, t38188: F, t38191: F, t38192: F, t38193: F, t38194: F, t8350: F, t8356: F, t9722: F, t9725: F, t9728: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43375 = 0.58540737209111952978e0 * t40332;
    let t43492 = 0.2927036860455597649e0 * t40623;
    let t43528 = 0.19513579069703984327e0 * t41191;
    let t43558 = 0.77886770749688743854e-2 * t41247;
    let t43566 = 0.2927036860455597649e0 * t41257;
    let t43571 = 0.64905642291407286545e-2 * t41265;
    let t43615 = 0.88895193539762595267e-1 * t41355;
    let t43622 = 0.66671395154821946449e-1 * t41363;
    let t43623 = 0.17740875559651324989e-2 * t41365;
    let t43677 = 0.66211599834018861287e-4 * t40681;
    let t43839 = 0.66211599834018861287e-4 * t41767;
    let t43990 = 0.87811105813667929469e0 * t42057;
    let t44083 = 0.58540737209111952978e0 * t40944;
    let t44085 = 0.87811105813667929469e0 * t40949;
    let t44089 = 0.2927036860455597649e0 * t40951;
    let t44512 = -t9722 - t9725 - t9728 + t38187 - t38188 - t38191 - 0.60975299583150056628e-3 * t8350 - t38192 - 0.60975299583150056628e-3 * t8356 - t38193 - t38194;
    (t43375, t43492, t43528, t43558, t43566, t43571, t43615, t43622, t43623, t43677, t43839, t43990, t44083, t44085, t44089, t44512)
}
