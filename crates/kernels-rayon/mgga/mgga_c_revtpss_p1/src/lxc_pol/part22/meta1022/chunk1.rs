//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3562/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3562(t1078: f64, t6258: f64, t3057: f64, t6343: f64, t1076: f64, t16284: f64, t16292: f64, t16295: f64, t16302: f64, t16312: f64, t16313: f64, t16327: f64, t16352: f64, t16371: f64, t20188: f64, t3060: f64, t3066: f64, t3269: f64, t3325: f64, t42052: f64, t4764: f64, t4773: f64, t4778: f64, t4941: f64, t5016: f64, t6392: f64) -> f64 {
    let t68018 = t1078 * t6258;
    let t68022 = t3057 * t6343;
    let t68038 = 0.26341796731742046394e1_f64 * t16302 * t4941 + 0.52683593463484092788e1_f64 * t16284 * t16292 + 0.13170898365871023197e1_f64 * t1076 * t3269 * t6392 * t3325 - 0.26341796731742046394e1_f64 * t16302 * t4773 - 0.26341796731742046394e1_f64 * t16312 * t68018 * t3066 + 0.13170898365871023197e1_f64 * t68022 * t3060 - 0.52683593463484092788e1_f64 * t16312 * t16313 * t16327 - 0.26341796731742046394e1_f64 * t16371 * t5016 - 0.79025390195226139182e1_f64 * t42052 * t20188 + 0.26341796731742046394e1_f64 * t16302 * t4764 + 0.26341796731742046394e1_f64 * t16284 * t16295 + 0.13170898365871023197e1_f64 * t4778 * t16352;
    t68038
}
