//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3777/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3777<F: Float>(t11249: F, t6688: F, t12717: F, t1287: F, t12966: F, t16771: F, t17188: F, t17307: F, t1770: F, t17808: F, t17829: F, t17887: F, t17905: F, t17917: F, t17951: F, t17958: F, t21518: F, t21524: F, t21583: F, t21599: F, t3670: F, t45707: F, t45718: F, t45738: F, t45852: F, t5446: F, t5466: F, t5486: F, t57264: F, t59671: F, t59686: F, t59817: F, t60037: F, t71945: F) -> (F, F) {
    let t72303 = t6688 * t11249;
    let t72315 = -F::cast_from(0.13170898365871023197e1_f64) * t17958 * t17905 + F::cast_from(0.26341796731742046394e1_f64) * t45707 * t21599 + F::cast_from(0.79025390195226139182e1_f64) * t59671 * t21583 + F::cast_from(0.52683593463484092788e1_f64) * t3670 * t5486 * t16771 + F::cast_from(0.26341796731742046394e1_f64) * t45852 * t21599 + F::cast_from(0.13170898365871023197e1_f64) * t12717 * t71945 * t1287 + F::cast_from(0.52683593463484092788e1_f64) * t1770 * t17887 * t5466 + F::cast_from(0.52683593463484092788e1_f64) * t12966 * t21524 + F::cast_from(0.52683593463484092788e1_f64) * t59817 * t17188 - F::cast_from(0.15805078039045227836e2_f64) * t57264 * t60037 * t16771 + F::cast_from(0.13170898365871023197e1_f64) * t1770 * t17808 - F::cast_from(0.13170898365871023197e1_f64) * t45738 * t72303 * t17951 + F::cast_from(0.26341796731742046394e1_f64) * t17307 * t17917 - F::cast_from(0.26341796731742046394e1_f64) * t17958 * t17829 - F::cast_from(0.26341796731742046394e1_f64) * t59686 * t5446 + F::cast_from(0.26341796731742046394e1_f64) * t45718 * t21518;
    (t72303, t72315)
}
