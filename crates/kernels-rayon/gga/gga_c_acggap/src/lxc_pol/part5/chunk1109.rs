//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1109/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1109(t1922: f64, t980: f64, t381: f64, t6454: f64, t879: f64, t1170: f64, t12326: f64, t12328: f64, t12344: f64, t18989: f64, t19000: f64, t19757: f64, t19862: f64, t19864: f64, t19870: f64, t19874: f64, t19880: f64, t407: f64, t6465: f64, t930: f64) -> f64 {
    let t19882 = t980 * t1922;
    let t19885 = t381 * t6454 * t879;
    let t19887 = -0.65854491829355115987e0_f64 * t1170 * t6465 * t930 + 0.65854491829355115987e0_f64 * t19862 + 0.13170898365871023197e1_f64 * t19864 + 0.13170898365871023197e1_f64 * t12326 - 0.39512695097613069592e1_f64 * t12328 + 0.13170898365871023197e1_f64 * t18989 + 0.26341796731742046394e1_f64 * t19870 + 0.13170898365871023197e1_f64 * t19874 - 0.26341796731742046394e1_f64 * t1170 * t19757 * t407 + 0.52683593463484092788e1_f64 * t19000 + 0.26341796731742046394e1_f64 * t19880 + 0.13170898365871023197e1_f64 * t19882 + t12344 - 0.65854491829355115987e0_f64 * t19885;
    t19887
}
