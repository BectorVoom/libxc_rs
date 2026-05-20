//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3792/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3792<F: Float>(t12641: F, t13177: F, t17974: F, t17992: F, t18030: F, t18102: F, t18114: F, t1829: F, t20700: F, t20710: F, t20714: F, t20744: F, t21408: F, t3556: F, t3729: F, t3732: F, t3791: F, t5237: F, t5417: F, t56303: F, t56327: F, t56332: F, t56432: F, t6564: F, t6588: F) -> F {
    let t72925 = F::cast_from(0.13170898365871023197e1_f64) * t3556 * t20710 - F::cast_from(0.79025390195226139182e1_f64) * t56332 * t18030 - F::cast_from(0.13170898365871023197e1_f64) * t13177 * t6588 + F::cast_from(0.79025390195226139182e1_f64) * t56327 * t17974 * t18102 + F::cast_from(0.65854491829355115987e0_f64) * t6564 * t3729 + F::cast_from(0.26341796731742046394e1_f64) * t18114 * t5237 + F::cast_from(0.26341796731742046394e1_f64) * t5417 * t17992 - F::cast_from(0.65854491829355115987e0_f64) * t20700 * t3791 + F::cast_from(0.52683593463484092788e1_f64) * t3732 * t21408 - F::cast_from(0.26341796731742046394e1_f64) * t12641 * t20714 - F::cast_from(0.13170898365871023197e1_f64) * t56303 * t1829 - F::cast_from(0.52683593463484092788e1_f64) * t56432 * t20744;
    t72925
}
