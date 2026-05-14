//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 994/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk994<F: Float>(t377: F, t6503: F, t1603: F, t524: F, t1170: F, t12305: F, t12307: F, t12310: F, t12315: F, t12345: F, t151: F, t1530: F, t1533: F, t1629: F, t1815: F, t18930: F, t18935: F, t18938: F, t18941: F, t19718: F, t407: F, t5080: F, t6461: F, t930: F) -> (F, F) {
    let t19802 = t377 * t6503;
    let t19807 = t1603 * t524;
    let t19824 = -0.79025390195226139182e1 * t18930 + 0.79025390195226139182e1 * t1530 * t19718 * t1533 - 0.13170898365871023197e1 * t19802 - 0.13170898365871023197e1 * t1170 * t1629 * t5080 - 0.26341796731742046394e1 * t1170 * t19807 * t407 - 0.10536718692696818558e2 * t18935 + 0.10536718692696818558e2 * t18938 + 0.26341796731742046394e1 * t18941 - 0.13170898365871023197e1 * t12305 + 0.13170898365871023197e1 * t12307 - 0.26341796731742046394e1 * t12310 + 0.26341796731742046394e1 * t12315 + 0.13170898365871023197e1 * t151 * t12345 * t1815 - 0.13170898365871023197e1 * t1170 * t6461 * t930;
    (t19807, t19824)
}
