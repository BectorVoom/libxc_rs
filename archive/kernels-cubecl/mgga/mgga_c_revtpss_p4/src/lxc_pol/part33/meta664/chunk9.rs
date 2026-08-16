//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2172/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2172<F: Float>(t108278: F, t786: F, t7286: F, t27989: F, t97802: F, t213: F, t30055: F, t1444: F, t1445: F, t25930: F, t27837: F, t27864: F, t27896: F, t27909: F, t30016: F, t5775: F, t7295: F, t94656: F, t94761: F, t94772: F, t94777: F, t94779: F, t97875: F, t97976: F, t97985: F, t98001: F, t98003: F) -> F {
    let t108379 = t786 * t108278;
    let t108380 = t108379 * t7286;
    let t108389 = t97802 * t27989;
    let t108395 = t213 * t30055;
    let t108399 = -t97976 - t94761 + F::cast_from(0.17347256376410398924e1_f64) * t27837 * t27896 - F::cast_from(0.34270468708064099208e-2_f64) * t94772 + F::cast_from(0.34270468708064099208e-1_f64) * t97985 + F::cast_from(0.72280234901709995518e-2_f64) * t108380 + F::cast_from(0.10408353825846239354e2_f64) * t7295 * t94656 * t30016 * t1444 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t97875 * t27864 - F::cast_from(0.14456046980341999104e-1_f64) * t108389 - F::cast_from(0.22849835011101738147e-2_f64) * t94777 - F::cast_from(0.13170898365871023197e1_f64) * t27909 * t5775 - F::cast_from(0.34270468708064099208e-1_f64) * t94779 - F::cast_from(0.65854491829355115987e0_f64) * t108395 * t1445 - t98001 - F::cast_from(0.4818682326780666368e-3_f64) * t98003;
    t108399
}
