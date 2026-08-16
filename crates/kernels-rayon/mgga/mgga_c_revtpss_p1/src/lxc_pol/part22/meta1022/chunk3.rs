//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3564/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3564(t3046: f64, t6343: f64, t1000: f64, t1073: f64, t1076: f64, t1096: f64, t16243: f64, t16333: f64, t16340: f64, t16344: f64, t16352: f64, t1652: f64, t16600: f64, t19856: f64, t20151: f64, t20204: f64, t3067: f64, t3269: f64, t386: f64, t4747: f64, t4758: f64, t4947: f64, t5016: f64, t53223: f64, t55464: f64, t65057: f64) -> f64 {
    let t68072 = t3046 * t6343;
    let t68097 = 0.26341796731742046394e1_f64 * t4747 * t16243 - 0.13170898365871023197e1_f64 * t68072 * t1000 + 0.65854491829355115987e0_f64 * t65057 * t386 + 0.13170898365871023197e1_f64 * t19856 * t1073 + 0.26341796731742046394e1_f64 * t1076 * t3269 * t20151 * t1096 + 0.52683593463484092788e1_f64 * t55464 * t4758 + 0.13170898365871023197e1_f64 * t4747 * t16352 + 0.52683593463484092788e1_f64 * t16340 * t4947 + 0.13170898365871023197e1_f64 * t20204 * t3067 - 0.26341796731742046394e1_f64 * t16333 * t5016 - 0.13170898365871023197e1_f64 * t53223 * t1652 - 0.26341796731742046394e1_f64 * t16600 * t16344;
    t68097
}
