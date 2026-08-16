//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1714/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1714<F: Float>(t1024: F, t1082: F, t1087: F, t1089: F, t11940: F, t12167: F, t12168: F, t19463: F, t19556: F, t19566: F, t24167: F, t3299: F, t3304: F, t378: F, t381: F, t4857: F, t6235: F, t6258: F, t6371: F, t6375: F, t6383: F, t6389: F, t67725: F, t88646: F, t88675: F, t88998: F, t89490: F, t89503: F) -> F {
    let t89536 = -F::cast_from(0.39512695097613069592e1_f64) * t1024 * t19556 * t6258 + F::cast_from(0.39512695097613069592e1_f64) * t19566 * t6383 + F::cast_from(0.79025390195226139183e1_f64) * t67725 * t6375 - F::cast_from(0.23707617058567841754e2_f64) * t11940 * t1082 * t88646 + F::cast_from(0.39512695097613069592e1_f64) * t6235 * t6389 - F::cast_from(0.79025390195226139183e1_f64) * t4857 * t24167 + F::cast_from(0.65854491829355115987e0_f64) * t88675 * t381 + F::cast_from(0.39512695097613069591e1_f64) * t3299 * t89490 * t3304 - F::cast_from(0.39512695097613069592e1_f64) * t19463 * t6371 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t378 * t88998 * t1089 + F::cast_from(0.15805078039045227836e2_f64) * t12167 * t89503 * t12168;
    t89536
}
