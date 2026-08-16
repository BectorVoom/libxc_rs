//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3780/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3780<F: Float>(t1770: F, t17845: F, t17852: F, t17948: F, t1204: F, t12709: F, t12717: F, t17821: F, t17849: F, t17856: F, t17875: F, t17876: F, t17888: F, t17944: F, t17952: F, t20800: F, t21040: F, t21298: F, t21451: F, t21465: F, t21491: F, t21521: F, t21579: F, t21583: F, t3670: F, t44421: F, t45385: F, t45700: F, t5230: F, t5458: F, t5459: F, t5466: F, t5478: F, t59686: F, t60013: F, t6714: F, t6717: F, t73: F) -> F {
    let t72429 = t1770 * t17845;
    let t72432 = t1770 * t17852;
    let t72435 = t1770 * t17948;
    let t72449 = F::cast_from(0.52683593463484092788e1_f64) * t1204 * t21451 * t5466 + F::cast_from(0.13170898365871023197e1_f64) * t44421 * t6714 - F::cast_from(0.79025390195226139182e1_f64) * t45385 * t21521 + F::cast_from(0.13170898365871023197e1_f64) * t12717 * t21040 * t17944 + F::cast_from(0.26341796731742046394e1_f64) * t17888 * t21465 - F::cast_from(0.13170898365871023197e1_f64) * t21579 * t17876 - F::cast_from(0.65854491829355115987e0_f64) * t5478 * t20800 * t17875 - F::cast_from(0.26341796731742046394e1_f64) * t45700 * t6717 - F::cast_from(0.26341796731742046394e1_f64) * t12709 * t21491 + F::cast_from(0.79025390195226139182e1_f64) * t72429 * t17849 - F::cast_from(0.79025390195226139182e1_f64) * t72432 * t17856 + F::cast_from(0.13170898365871023197e1_f64) * t72435 * t17952 + F::cast_from(0.79025390195226139182e1_f64) * t60013 * t21583 + F::cast_from(0.52683593463484092788e1_f64) * t3670 * t17821 * t5230 - F::cast_from(0.26341796731742046394e1_f64) * t59686 * t5459 + F::cast_from(0.26341796731742046394e1_f64) * t12717 * t21298 * t73 * t5458;
    t72449
}
