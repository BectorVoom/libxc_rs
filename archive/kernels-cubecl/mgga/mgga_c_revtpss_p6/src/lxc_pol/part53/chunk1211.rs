//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1211/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1211<F: Float>(t60221: F, t8736: F, t13272: F, t32805: F, t10301: F, t34409: F, t2247: F, t29362: F, t8435: F, t60224: F, t10309: F, t122893: F, t122911: F, t122918: F, t125238: F, t125294: F, t128368: F, t128371: F, t128374: F, t128377: F, t128444: F, t32151: F, t32586: F, t32602: F, t32795: F, t32798: F, t32802: F, t32806: F, t33621: F, t34173: F, t34402: F, t34410: F, t8623: F, t8737: F) -> F {
    let t129157 = t60221 * t8736;
    let t129160 = t13272 * t32805;
    let t129165 = t10301 * t34409;
    let t129169 = t2247 * t8435 * t29362;
    let t129180 = t60224 * t8736;
    let t129193 = t10309 * t34409;
    let t129196 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t122893 * t128368 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t122893 * t128371 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32802 * t128374 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32802 * t128377 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t129157 * t8623 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t129160 * t8623 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t34402 * t32151 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t129165 * t8623 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t129169 * t8623 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t34410 * t32151 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t32795 * t33621 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t32806 * t33621 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t8737 * t125238 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t129180 * t32586 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t34402 * t32602 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t122911 * t34173 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t122918 * t34173 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t32798 * t125294 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t32798 * t128444 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t129193 * t32586;
    t129196
}
