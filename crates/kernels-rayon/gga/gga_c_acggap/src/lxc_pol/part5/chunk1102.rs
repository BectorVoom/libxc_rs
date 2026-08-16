//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1102/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1102(t3088: f64, t4183: f64, t6465: f64, t3077: f64, t6462: f64, t1170: f64, t1530: f64, t1533: f64, t18895: f64, t18897: f64, t18910: f64, t19741: f64, t19743: f64, t19746: f64, t19748: f64, t19752: f64, t19757: f64, t3084: f64, t407: f64, t6461: f64, t6482: f64, t955: f64) -> f64 {
    let t19769 = t3088 * t6465 * t4183;
    let t19771 = t3077 * t6462;
    let t19773 = -0.79025390195226139182e1_f64 * t18895 - 0.13170898365871023197e1_f64 * t18897 - 0.26341796731742046394e1_f64 * t19741 - 0.13170898365871023197e1_f64 * t19743 + 0.13170898365871023197e1_f64 * t19746 - 0.13170898365871023197e1_f64 * t1170 * t19748 * t407 + 0.13170898365871023197e1_f64 * t19752 + 0.26341796731742046394e1_f64 * t1530 * t19748 * t1533 + 0.52683593463484092788e1_f64 * t1530 * t19757 * t1533 + 0.26341796731742046394e1_f64 * t1530 * t6461 * t3084 - 0.13170898365871023197e1_f64 * t18910 - 0.65854491829355115987e0_f64 * t1170 * t6482 * t955 - 0.13170898365871023197e1_f64 * t19769 + 0.26341796731742046394e1_f64 * t19771;
    t19773
}
