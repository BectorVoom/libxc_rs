//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1225/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1225<F: Float>(t2217: F, t939: F, t1614: F, t8114: F, t556: F, t8306: F, t32003: F, t4210: F, t8065: F, t8998: F, t2236: F, t2241: F, t32124: F, t33015: F, t33019: F, t33024: F, t33028: F, t33031: F, t33566: F, t33651: F, t33727: F, t7931: F, t7934: F, t8400: F, t8791: F, t9402: F) -> (F, F) {
    let t38046 = t939 * t2217;
    let t38051 = F::cast_from(0.13170898365871023197e1_f64) * t8114 * t1614;
    let t38052 = t8306 * t556;
    let t38055 = F::cast_from(0.34694512752820797848e1_f64) * t32003 * t38052 * t4210;
    let t38065 = t8998 * t8065;
    let t38072 = -F::cast_from(0.17347256376410398924e1_f64) * t8400 * t38046 * t8791 + t33015 + t33019 + t38051 + t38055 + F::cast_from(0.52041769129231196772e1_f64) * t32124 * t38052 * t7934 - F::cast_from(0.8673628188205199462e0_f64) * t7931 * t8306 * t33651 + F::cast_from(0.34694512752820797848e1_f64) * t33024 + F::cast_from(0.8673628188205199462e0_f64) * t33566 * t2241 + F::cast_from(0.8673628188205199462e0_f64) * t38065 + F::cast_from(0.8673628188205199462e0_f64) * t33727 * t9402 + F::cast_from(0.69389025505641595696e1_f64) * t33028 + t33031 + F::cast_from(0.17347256376410398924e1_f64) * t33566 * t2236;
    (t38052, t38072)
}
