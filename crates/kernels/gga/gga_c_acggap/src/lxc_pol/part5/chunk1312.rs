//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1312/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1312<F: Float>(t3378: F, t6532: F, t1629: F, t22394: F, t3073: F, t1160: F, t1539: F, t20228: F, t1170: F, t14572: F, t14577: F, t14579: F, t151: F, t1530: F, t19135: F, t19249: F, t19252: F, t19255: F, t19262: F, t4198: F, t4199: F, t525: F, t6465: F, t6482: F, t945: F, t955: F) -> F {
    let t24461 = t3378 * t6532;
    let t24464 = t3073 * t1629 * t22394;
    let t24467 = t1160 * t20228 * t1539;
    let t24469 = F::new(0.26341796731742046394e1) * t19249 + F::new(0.13170898365871023197e1) * t19252 - F::new(0.13170898365871023197e1) * t19255 - F::new(0.52683593463484092788e1) * t19262 - F::new(0.65854491829355115987e0) * t1170 * t6465 * t955 + F::new(0.13170898365871023197e1) * t14572 - F::new(0.39512695097613069591e1) * t4198 * t6482 * t4199 + F::new(0.39512695097613069591e1) * t1530 * t6482 * t945 - F::new(0.79025390195226139182e1) * t14577 + F::new(0.13170898365871023197e1) * t14579 - F::new(0.13170898365871023197e1) * t151 * t19135 * t525 - F::new(0.13170898365871023197e1) * t24461 - F::new(0.52683593463484092788e1) * t24464 + F::new(0.13170898365871023197e1) * t24467;
    t24469
}
