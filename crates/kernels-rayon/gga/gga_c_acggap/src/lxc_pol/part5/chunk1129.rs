//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1129/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1129(t377: f64, t6518: f64, t159: f64, t322: f64, t381: f64, t6413: f64, t12334: f64, t943: f64, t310: f64, t6515: f64, t12401: f64, t14459: f64, t14460: f64, t14478: f64, t18750: f64, t19090: f64, t19095: f64, t19098: f64, t19103: f64, t4198: f64, t4199: f64, t6461: f64, t6465: f64) -> (f64, f64) {
    let t20190 = t377 * t6518;
    let t20203 = t381 * t159 * t6413 * t322;
    let t20206 = t12334 * t943;
    let t20213 = t310 * t6515;
    let t20216 = 0.13170898365871023197e1_f64 * t19090 - t14459 - 0.13170898365871023197e1_f64 * t20190 - 0.65854491829355115987e0_f64 * t14460 - 0.39512695097613069591e1_f64 * t4198 * t6465 * t12401 + 0.52683593463484092788e1_f64 * t19095 - 0.79025390195226139182e1_f64 * t4198 * t6461 * t4199 + 0.13170898365871023197e1_f64 * t19098 - 0.13170898365871023197e1_f64 * t20203 - 0.13170898365871023197e1_f64 * t19103 + 0.15805078039045227836e2_f64 * t18750 * t6465 * t20206 - 0.23707617058567841754e2_f64 * t4198 * t6465 * t4199 + 0.13170898365871023197e1_f64 * t20213 - 0.13170898365871023197e1_f64 * t14478;
    (t20206, t20216)
}
