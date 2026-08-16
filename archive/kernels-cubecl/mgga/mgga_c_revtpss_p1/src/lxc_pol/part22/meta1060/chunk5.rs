//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3776/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3776<F: Float>(t1209: F, t1284: F, t6695: F, t20849: F, t3754: F, t12709: F, t12987: F, t16697: F, t17345: F, t17883: F, t17888: F, t17893: F, t17902: F, t17949: F, t17951: F, t17958: F, t1822: F, t21040: F, t21473: F, t21480: F, t21587: F, t21596: F, t3601: F, t3755: F, t3756: F, t45718: F, t5459: F, t5486: F, t57465: F, t59537: F, t59681: F, t59864: F, t59865: F, t59871: F, t59872: F, t59987: F, t60008: F, t70235: F, t70890: F) -> F {
    let t72267 = t1209 * t1284 * t6695;
    let t72270 = t20849 * t3754;
    let t72276 = -F::cast_from(0.79025390195226139182e1_f64) * t57465 * t17893 - F::cast_from(0.26341796731742046394e1_f64) * t60008 * t5459 + F::cast_from(0.15805078039045227836e2_f64) * t59864 * t70235 * t59865 * t3601 - F::cast_from(0.23707617058567841754e2_f64) * t59871 * t70235 * t59872 * t3601 - F::cast_from(0.79025390195226139182e1_f64) * t59987 * t21587 - F::cast_from(0.79025390195226139182e1_f64) * t12987 * t5486 * t17345 + F::cast_from(0.13170898365871023197e1_f64) * t59537 * t1822 + F::cast_from(0.65854491829355115987e0_f64) * t17949 * t70890 * t17951 - F::cast_from(0.13170898365871023197e1_f64) * t12709 * t21480 - F::cast_from(0.26341796731742046394e1_f64) * t17958 * t17902 + F::cast_from(0.13170898365871023197e1_f64) * t45718 * t21473 + F::cast_from(0.26341796731742046394e1_f64) * t59681 * t16697 + F::cast_from(0.52683593463484092788e1_f64) * t17888 * t21596 - F::cast_from(0.13170898365871023197e1_f64) * t72267 * t3756 - F::cast_from(0.13170898365871023197e1_f64) * t72270 * t3756 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t21040 * t17883;
    t72276
}
