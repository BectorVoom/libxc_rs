//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1714/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1714(t1024: f64, t1082: f64, t1087: f64, t1089: f64, t11940: f64, t12167: f64, t12168: f64, t19463: f64, t19556: f64, t19566: f64, t24167: f64, t3299: f64, t3304: f64, t378: f64, t381: f64, t4857: f64, t6235: f64, t6258: f64, t6371: f64, t6375: f64, t6383: f64, t6389: f64, t67725: f64, t88646: f64, t88675: f64, t88998: f64, t89490: f64, t89503: f64) -> f64 {
    let t89536 = -0.39512695097613069592e1_f64 * t1024 * t19556 * t6258 + 0.39512695097613069592e1_f64 * t19566 * t6383 + 0.79025390195226139183e1_f64 * t67725 * t6375 - 0.23707617058567841754e2_f64 * t11940 * t1082 * t88646 + 0.39512695097613069592e1_f64 * t6235 * t6389 - 0.79025390195226139183e1_f64 * t4857 * t24167 + 0.65854491829355115987e0_f64 * t88675 * t381 + 0.39512695097613069591e1_f64 * t3299 * t89490 * t3304 - 0.39512695097613069592e1_f64 * t19463 * t6371 + 0.65854491829355115987e0_f64 * t1087 * t378 * t88998 * t1089 + 0.15805078039045227836e2_f64 * t12167 * t89503 * t12168;
    t89536
}
