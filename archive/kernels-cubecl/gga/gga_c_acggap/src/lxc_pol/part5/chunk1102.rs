//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1102/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1102<F: Float>(t3088: F, t4183: F, t6465: F, t3077: F, t6462: F, t1170: F, t1530: F, t1533: F, t18895: F, t18897: F, t18910: F, t19741: F, t19743: F, t19746: F, t19748: F, t19752: F, t19757: F, t3084: F, t407: F, t6461: F, t6482: F, t955: F) -> F {
    let t19769 = t3088 * t6465 * t4183;
    let t19771 = t3077 * t6462;
    let t19773 = -F::cast_from(0.79025390195226139182e1_f64) * t18895 - F::cast_from(0.13170898365871023197e1_f64) * t18897 - F::cast_from(0.26341796731742046394e1_f64) * t19741 - F::cast_from(0.13170898365871023197e1_f64) * t19743 + F::cast_from(0.13170898365871023197e1_f64) * t19746 - F::cast_from(0.13170898365871023197e1_f64) * t1170 * t19748 * t407 + F::cast_from(0.13170898365871023197e1_f64) * t19752 + F::cast_from(0.26341796731742046394e1_f64) * t1530 * t19748 * t1533 + F::cast_from(0.52683593463484092788e1_f64) * t1530 * t19757 * t1533 + F::cast_from(0.26341796731742046394e1_f64) * t1530 * t6461 * t3084 - F::cast_from(0.13170898365871023197e1_f64) * t18910 - F::cast_from(0.65854491829355115987e0_f64) * t1170 * t6482 * t955 - F::cast_from(0.13170898365871023197e1_f64) * t19769 + F::cast_from(0.26341796731742046394e1_f64) * t19771;
    t19773
}
