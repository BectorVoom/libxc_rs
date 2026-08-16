//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1095/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1095(t1659: f64, t4137: f64, t5517: f64, t852: f64, t868: f64, t12282: f64, t1265: f64, t15221: f64, t15223: f64, t19607: f64, t19611: f64, t19615: f64, t19618: f64, t5520: f64) -> f64 {
    let t19620 = t4137 * t1659;
    let t19627 = t852 * t5517;
    let t19629 = t868 * t5517;
    let t19631 = -0.26341796731742046394e1_f64 * t19607 + 0.26341796731742046394e1_f64 * t19611 + 0.13170898365871023197e1_f64 * t19615 + 0.13170898365871023197e1_f64 * t19618 - 0.26341796731742046394e1_f64 * t19620 + 0.13170898365871023197e1_f64 * t12282 - 0.79025390195226139182e1_f64 * t15221 - 0.13170898365871023197e1_f64 * t15223 - 0.65854491829355115987e0_f64 * t5520 * t1265 - 0.26341796731742046394e1_f64 * t19627 - 0.26341796731742046394e1_f64 * t19629;
    t19631
}
