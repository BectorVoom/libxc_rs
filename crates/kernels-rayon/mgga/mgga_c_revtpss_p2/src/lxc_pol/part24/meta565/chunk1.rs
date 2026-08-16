//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1715/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1715(t1087: f64, t1089: f64, t12078: f64, t12079: f64, t16502: f64, t16584: f64, t1678: f64, t19463: f64, t19566: f64, t23820: f64, t24098: f64, t24132: f64, t24138: f64, t24141: f64, t24152: f64, t3287: f64, t342: f64, t380: f64, t4857: f64, t55988: f64, t55991: f64, t6368: f64, t6379: f64, t89245: f64, t89355: f64, t89503: f64) -> f64 {
    let t89565 = -0.26341796731742046395e1_f64 * t3287 * t89245 * t1089 - 0.15805078039045227836e2_f64 * t12078 * t89503 * t12079 - 0.15805078039045227836e2_f64 * t55988 * t24138 + 0.79025390195226139183e1_f64 * t55991 * t24141 - 0.79025390195226139183e1_f64 * t16502 * t24132 + 0.65854491829355115987e0_f64 * t342 * t380 * t89355 - 0.79025390195226139183e1_f64 * t19463 * t6368 - 0.79025390195226139183e1_f64 * t16584 * t24152 + 0.79025390195226139183e1_f64 * t19566 * t6379 - 0.79025390195226139183e1_f64 * t4857 * t24098 + 0.26341796731742046395e1_f64 * t1087 * t1678 * t23820 * t1089;
    t89565
}
