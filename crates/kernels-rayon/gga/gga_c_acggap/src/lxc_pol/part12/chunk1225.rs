//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1225/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1225(t2217: f64, t939: f64, t1614: f64, t8114: f64, t556: f64, t8306: f64, t32003: f64, t4210: f64, t8065: f64, t8998: f64, t2236: f64, t2241: f64, t32124: f64, t33015: f64, t33019: f64, t33024: f64, t33028: f64, t33031: f64, t33566: f64, t33651: f64, t33727: f64, t7931: f64, t7934: f64, t8400: f64, t8791: f64, t9402: f64) -> (f64, f64) {
    let t38046 = t939 * t2217;
    let t38051 = 0.13170898365871023197e1_f64 * t8114 * t1614;
    let t38052 = t8306 * t556;
    let t38055 = 0.34694512752820797848e1_f64 * t32003 * t38052 * t4210;
    let t38065 = t8998 * t8065;
    let t38072 = -0.17347256376410398924e1_f64 * t8400 * t38046 * t8791 + t33015 + t33019 + t38051 + t38055 + 0.52041769129231196772e1_f64 * t32124 * t38052 * t7934 - 0.8673628188205199462e0_f64 * t7931 * t8306 * t33651 + 0.34694512752820797848e1_f64 * t33024 + 0.8673628188205199462e0_f64 * t33566 * t2241 + 0.8673628188205199462e0_f64 * t38065 + 0.8673628188205199462e0_f64 * t33727 * t9402 + 0.69389025505641595696e1_f64 * t33028 + t33031 + 0.17347256376410398924e1_f64 * t33566 * t2236;
    (t38052, t38072)
}
