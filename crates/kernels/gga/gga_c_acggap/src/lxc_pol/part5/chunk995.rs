//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 995/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk995<F: Float>(t1410: F, t407: F, t1931: F, t980: F, t377: F, t6552: F, t1160: F, t4210: F, t6465: F, t4180: F, t6483: F, t3088: F, t4183: F, t6482: F, t1170: F, t12318: F, t1530: F, t1629: F, t18951: F, t18953: F, t18957: F, t18977: F, t3084: F, t6461: F, t945: F, t955: F) -> (F, F) {
    let t19834 = t407 * t1410;
    let t19838 = t980 * t1931;
    let t19840 = t377 * t6552;
    let t19843 = t1160 * t6465 * t4210;
    let t19845 = t4180 * t6483;
    let t19854 = t3088 * t6482 * t4183;
    let t19857 = -0.26341796731742046394e1 * t18951 + 0.52683593463484092788e1 * t18953 + 0.26341796731742046394e1 * t18957 - 0.79025390195226139182e1 * t12318 + 0.39512695097613069591e1 * t1530 * t6465 * t3084 - 0.26341796731742046394e1 * t1170 * t1629 * t19834 - 0.65854491829355115987e0 * t19838 - 0.13170898365871023197e1 * t19840 + 0.13170898365871023197e1 * t19843 + 0.26341796731742046394e1 * t19845 + 0.92196288561097162379e1 * t1530 * t6465 * t945 - 0.13170898365871023197e1 * t1170 * t6461 * t955 - 0.13170898365871023197e1 * t19854 + 0.52683593463484092788e1 * t18977;
    (t19834, t19857)
}
