//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1107/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1107<F: Float>(t1170: F, t12318: F, t1530: F, t1629: F, t18951: F, t18953: F, t18957: F, t18977: F, t19834: F, t19838: F, t19840: F, t19843: F, t19845: F, t19854: F, t3084: F, t6461: F, t6465: F, t945: F, t955: F) -> F {
    let t19857 = -F::cast_from(0.26341796731742046394e1_f64) * t18951 + F::cast_from(0.52683593463484092788e1_f64) * t18953 + F::cast_from(0.26341796731742046394e1_f64) * t18957 - F::cast_from(0.79025390195226139182e1_f64) * t12318 + F::cast_from(0.39512695097613069591e1_f64) * t1530 * t6465 * t3084 - F::cast_from(0.26341796731742046394e1_f64) * t1170 * t1629 * t19834 - F::cast_from(0.65854491829355115987e0_f64) * t19838 - F::cast_from(0.13170898365871023197e1_f64) * t19840 + F::cast_from(0.13170898365871023197e1_f64) * t19843 + F::cast_from(0.26341796731742046394e1_f64) * t19845 + F::cast_from(0.92196288561097162379e1_f64) * t1530 * t6465 * t945 - F::cast_from(0.13170898365871023197e1_f64) * t1170 * t6461 * t955 - F::cast_from(0.13170898365871023197e1_f64) * t19854 + F::cast_from(0.52683593463484092788e1_f64) * t18977;
    t19857
}
