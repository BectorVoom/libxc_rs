//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2200/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2200(t108278: f64, t786: f64, t7286: f64, t27989: f64, t97802: f64, t213: f64, t30055: f64, t1444: f64, t1445: f64, t25930: f64, t27837: f64, t27864: f64, t27896: f64, t27909: f64, t30016: f64, t5775: f64, t7295: f64, t94656: f64, t94761: f64, t94772: f64, t94777: f64, t94779: f64, t97875: f64, t97976: f64, t97985: f64, t98001: f64, t98003: f64) -> f64 {
    let t108379 = t786 * t108278;
    let t108380 = t108379 * t7286;
    let t108389 = t97802 * t27989;
    let t108395 = t213 * t30055;
    let t108399 = -t97976 - t94761 + 0.17347256376410398924e1_f64 * t27837 * t27896 - 0.34270468708064099208e-2_f64 * t94772 + 0.34270468708064099208e-1_f64 * t97985 + 0.72280234901709995518e-2_f64 * t108380 + 0.10408353825846239354e2_f64 * t7295 * t94656 * t30016 * t1444 - 0.17347256376410398924e1_f64 * t25930 * t97875 * t27864 - 0.14456046980341999104e-1_f64 * t108389 - 0.22849835011101738147e-2_f64 * t94777 - 0.13170898365871023197e1_f64 * t27909 * t5775 - 0.34270468708064099208e-1_f64 * t94779 - 0.65854491829355115987e0_f64 * t108395 * t1445 - t98001 - 0.4818682326780666368e-3_f64 * t98003;
    t108399
}
