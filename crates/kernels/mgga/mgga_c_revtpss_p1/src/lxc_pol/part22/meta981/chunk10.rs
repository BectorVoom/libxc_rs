//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3321/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3321<F: Float>(t10542: F, t18726: F, t14546: F, t14547: F, t18677: F, t4514: F, t51646: F, t51653: F, t51657: F, t51660: F, t51668: F, t51672: F, t51676: F, t51680: F, t51682: F, t62385: F, t62612: F, t820: F, t837: F, t879: F) -> F {
    let t63015 = t10542 * t18726;
    let t63024 = -F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t62385 - F::cast_from(0.92526556154787137113e-2_f64) * t51646 + F::cast_from(0.21951497276451705328e-1_f64) * t51653 - F::cast_from(0.1040793657534163522e0_f64) * t51657 + F::cast_from(0.39274398764404314548e-3_f64) * t51660 - F::cast_from(0.11708928647259339623e0_f64) * t51668 + F::cast_from(0.10975748638225852664e-1_f64) * t51672 - F::cast_from(0.39274398764404314548e-3_f64) * t51676 + F::cast_from(0.21951497276451705328e-1_f64) * t51680 - F::cast_from(0.19514881078765566038e-1_f64) * t63015 - F::cast_from(0.13170898365871023197e1_f64) * t4514 * t62612 * t837 - F::cast_from(0.23707617058567841754e2_f64) * t14546 * t18677 * t14547 + F::cast_from(0.2601984143835408805e-2_f64) * t51682;
    t63024
}
