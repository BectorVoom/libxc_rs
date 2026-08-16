//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1107/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1107(t1170: f64, t12318: f64, t1530: f64, t1629: f64, t18951: f64, t18953: f64, t18957: f64, t18977: f64, t19834: f64, t19838: f64, t19840: f64, t19843: f64, t19845: f64, t19854: f64, t3084: f64, t6461: f64, t6465: f64, t945: f64, t955: f64) -> f64 {
    let t19857 = -0.26341796731742046394e1_f64 * t18951 + 0.52683593463484092788e1_f64 * t18953 + 0.26341796731742046394e1_f64 * t18957 - 0.79025390195226139182e1_f64 * t12318 + 0.39512695097613069591e1_f64 * t1530 * t6465 * t3084 - 0.26341796731742046394e1_f64 * t1170 * t1629 * t19834 - 0.65854491829355115987e0_f64 * t19838 - 0.13170898365871023197e1_f64 * t19840 + 0.13170898365871023197e1_f64 * t19843 + 0.26341796731742046394e1_f64 * t19845 + 0.92196288561097162379e1_f64 * t1530 * t6465 * t945 - 0.13170898365871023197e1_f64 * t1170 * t6461 * t955 - 0.13170898365871023197e1_f64 * t19854 + 0.52683593463484092788e1_f64 * t18977;
    t19857
}
