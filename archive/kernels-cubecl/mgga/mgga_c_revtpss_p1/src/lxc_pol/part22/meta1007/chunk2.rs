//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3445/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3445<F: Float>(t20112: F, t994: F, t1000: F, t1079: F, t11187: F, t15579: F, t16254: F, t16312: F, t16322: F, t16374: F, t16597: F, t16603: F, t19421: F, t19428: F, t20172: F, t20195: F, t3052: F, t3075: F, t3264: F, t4743: F, t4747: F, t4764: F, t4932: F, t4935: F, t4940: F, t53130: F, t6392: F, t995: F) -> F {
    let t64737 = t994 * t20112;
    let t64753 = -F::cast_from(0.26341796731742046394e1_f64) * t11187 * t19421 + F::cast_from(0.26341796731742046394e1_f64) * t16597 * t4764 - F::cast_from(0.79025390195226139182e1_f64) * t4935 * t16322 + F::cast_from(0.65854491829355115987e0_f64) * t995 * t1079 * t6392 * t3075 - F::cast_from(0.52683593463484092788e1_f64) * t16312 * t53130 * t4940 - F::cast_from(0.13170898365871023197e1_f64) * t64737 * t1000 + F::cast_from(0.26341796731742046394e1_f64) * t16374 * t4764 - F::cast_from(0.52683593463484092788e1_f64) * t16603 * t19428 * t16254 + F::cast_from(0.52683593463484092788e1_f64) * t3052 * t20195 + F::cast_from(0.26341796731742046394e1_f64) * t3264 * t20172 + F::cast_from(0.13170898365871023197e1_f64) * t4747 * t15579 + F::cast_from(0.26341796731742046394e1_f64) * t4743 * t4932;
    t64753
}
