//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1312/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1312(t3378: f64, t6532: f64, t1629: f64, t22394: f64, t3073: f64, t1160: f64, t1539: f64, t20228: f64, t1170: f64, t14572: f64, t14577: f64, t14579: f64, t151: f64, t1530: f64, t19135: f64, t19249: f64, t19252: f64, t19255: f64, t19262: f64, t4198: f64, t4199: f64, t525: f64, t6465: f64, t6482: f64, t945: f64, t955: f64) -> f64 {
    let t24461 = t3378 * t6532;
    let t24464 = t3073 * t1629 * t22394;
    let t24467 = t1160 * t20228 * t1539;
    let t24469 = 0.26341796731742046394e1_f64 * t19249 + 0.13170898365871023197e1_f64 * t19252 - 0.13170898365871023197e1_f64 * t19255 - 0.52683593463484092788e1_f64 * t19262 - 0.65854491829355115987e0_f64 * t1170 * t6465 * t955 + 0.13170898365871023197e1_f64 * t14572 - 0.39512695097613069591e1_f64 * t4198 * t6482 * t4199 + 0.39512695097613069591e1_f64 * t1530 * t6482 * t945 - 0.79025390195226139182e1_f64 * t14577 + 0.13170898365871023197e1_f64 * t14579 - 0.13170898365871023197e1_f64 * t151 * t19135 * t525 - 0.13170898365871023197e1_f64 * t24461 - 0.52683593463484092788e1_f64 * t24464 + 0.13170898365871023197e1_f64 * t24467;
    t24469
}
