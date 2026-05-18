//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1129/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1129<F: Float>(t377: F, t6518: F, t159: F, t322: F, t381: F, t6413: F, t12334: F, t943: F, t310: F, t6515: F, t12401: F, t14459: F, t14460: F, t14478: F, t18750: F, t19090: F, t19095: F, t19098: F, t19103: F, t4198: F, t4199: F, t6461: F, t6465: F) -> (F, F) {
    let t20190 = t377 * t6518;
    let t20203 = t381 * t159 * t6413 * t322;
    let t20206 = t12334 * t943;
    let t20213 = t310 * t6515;
    let t20216 = F::new(0.13170898365871023197e1) * t19090 - t14459 - F::new(0.13170898365871023197e1) * t20190 - F::new(0.65854491829355115987e0) * t14460 - F::new(0.39512695097613069591e1) * t4198 * t6465 * t12401 + F::new(0.52683593463484092788e1) * t19095 - F::new(0.79025390195226139182e1) * t4198 * t6461 * t4199 + F::new(0.13170898365871023197e1) * t19098 - F::new(0.13170898365871023197e1) * t20203 - F::new(0.13170898365871023197e1) * t19103 + F::new(0.15805078039045227836e2) * t18750 * t6465 * t20206 - F::new(0.23707617058567841754e2) * t4198 * t6465 * t4199 + F::new(0.13170898365871023197e1) * t20213 - F::new(0.13170898365871023197e1) * t14478;
    (t20206, t20216)
}
