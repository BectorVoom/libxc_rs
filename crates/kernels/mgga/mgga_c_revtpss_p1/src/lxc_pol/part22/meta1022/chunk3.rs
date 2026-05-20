//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3564/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3564<F: Float>(t3046: F, t6343: F, t1000: F, t1073: F, t1076: F, t1096: F, t16243: F, t16333: F, t16340: F, t16344: F, t16352: F, t1652: F, t16600: F, t19856: F, t20151: F, t20204: F, t3067: F, t3269: F, t386: F, t4747: F, t4758: F, t4947: F, t5016: F, t53223: F, t55464: F, t65057: F) -> F {
    let t68072 = t3046 * t6343;
    let t68097 = F::cast_from(0.26341796731742046394e1_f64) * t4747 * t16243 - F::cast_from(0.13170898365871023197e1_f64) * t68072 * t1000 + F::cast_from(0.65854491829355115987e0_f64) * t65057 * t386 + F::cast_from(0.13170898365871023197e1_f64) * t19856 * t1073 + F::cast_from(0.26341796731742046394e1_f64) * t1076 * t3269 * t20151 * t1096 + F::cast_from(0.52683593463484092788e1_f64) * t55464 * t4758 + F::cast_from(0.13170898365871023197e1_f64) * t4747 * t16352 + F::cast_from(0.52683593463484092788e1_f64) * t16340 * t4947 + F::cast_from(0.13170898365871023197e1_f64) * t20204 * t3067 - F::cast_from(0.26341796731742046394e1_f64) * t16333 * t5016 - F::cast_from(0.13170898365871023197e1_f64) * t53223 * t1652 - F::cast_from(0.26341796731742046394e1_f64) * t16600 * t16344;
    t68097
}
