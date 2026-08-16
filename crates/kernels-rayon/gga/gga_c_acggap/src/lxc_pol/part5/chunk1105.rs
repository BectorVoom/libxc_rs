//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1105/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1105(t377: f64, t6503: f64, t1603: f64, t524: f64, t1170: f64, t12305: f64, t12307: f64, t12310: f64, t12315: f64, t12345: f64, t151: f64, t1530: f64, t1533: f64, t1629: f64, t1815: f64, t18930: f64, t18935: f64, t18938: f64, t18941: f64, t19718: f64, t407: f64, t5080: f64, t6461: f64, t930: f64) -> (f64, f64) {
    let t19802 = t377 * t6503;
    let t19807 = t1603 * t524;
    let t19824 = -0.79025390195226139182e1_f64 * t18930 + 0.79025390195226139182e1_f64 * t1530 * t19718 * t1533 - 0.13170898365871023197e1_f64 * t19802 - 0.13170898365871023197e1_f64 * t1170 * t1629 * t5080 - 0.26341796731742046394e1_f64 * t1170 * t19807 * t407 - 0.10536718692696818558e2_f64 * t18935 + 0.10536718692696818558e2_f64 * t18938 + 0.26341796731742046394e1_f64 * t18941 - 0.13170898365871023197e1_f64 * t12305 + 0.13170898365871023197e1_f64 * t12307 - 0.26341796731742046394e1_f64 * t12310 + 0.26341796731742046394e1_f64 * t12315 + 0.13170898365871023197e1_f64 * t151 * t12345 * t1815 - 0.13170898365871023197e1_f64 * t1170 * t6461 * t930;
    (t19807, t19824)
}
