//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1100/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1100(t1160: f64, t4162: f64, t6465: f64, t12285: f64, t12286: f64, t12290: f64, t1530: f64, t18880: f64, t18884: f64, t18887: f64, t18890: f64, t18893: f64, t19716: f64, t19718: f64, t3084: f64, t4166: f64, t4198: f64, t5853: f64, t6263: f64, t6482: f64) -> f64 {
    let t19732 = t1160 * t6465 * t4162;
    let t19738 = -t12285 - 0.79025390195226139182e1_f64 * t19716 - 0.79025390195226139182e1_f64 * t4198 * t19718 * t5853 + 0.26341796731742046394e1_f64 * t12286 + 0.13170898365871023197e1_f64 * t12290 - 0.52683593463484092788e1_f64 * t18880 - 0.52683593463484092788e1_f64 * t18884 - 0.52683593463484092788e1_f64 * t18887 - 0.26341796731742046394e1_f64 * t18890 + 0.52683593463484092788e1_f64 * t1530 * t4166 * t6263 + 0.65854491829355115987e0_f64 * t19732 + 0.13170898365871023197e1_f64 * t1530 * t6482 * t3084 + 0.79025390195226139182e1_f64 * t18893;
    t19738
}
