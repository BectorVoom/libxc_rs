//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3562/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3562<F: Float>(t1078: F, t6258: F, t3057: F, t6343: F, t1076: F, t16284: F, t16292: F, t16295: F, t16302: F, t16312: F, t16313: F, t16327: F, t16352: F, t16371: F, t20188: F, t3060: F, t3066: F, t3269: F, t3325: F, t42052: F, t4764: F, t4773: F, t4778: F, t4941: F, t5016: F, t6392: F) -> F {
    let t68018 = t1078 * t6258;
    let t68022 = t3057 * t6343;
    let t68038 = F::cast_from(0.26341796731742046394e1_f64) * t16302 * t4941 + F::cast_from(0.52683593463484092788e1_f64) * t16284 * t16292 + F::cast_from(0.13170898365871023197e1_f64) * t1076 * t3269 * t6392 * t3325 - F::cast_from(0.26341796731742046394e1_f64) * t16302 * t4773 - F::cast_from(0.26341796731742046394e1_f64) * t16312 * t68018 * t3066 + F::cast_from(0.13170898365871023197e1_f64) * t68022 * t3060 - F::cast_from(0.52683593463484092788e1_f64) * t16312 * t16313 * t16327 - F::cast_from(0.26341796731742046394e1_f64) * t16371 * t5016 - F::cast_from(0.79025390195226139182e1_f64) * t42052 * t20188 + F::cast_from(0.26341796731742046394e1_f64) * t16302 * t4764 + F::cast_from(0.26341796731742046394e1_f64) * t16284 * t16295 + F::cast_from(0.13170898365871023197e1_f64) * t4778 * t16352;
    t68038
}
