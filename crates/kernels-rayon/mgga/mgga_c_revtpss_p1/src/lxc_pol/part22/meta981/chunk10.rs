//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3321/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3321(t10542: f64, t18726: f64, t14546: f64, t14547: f64, t18677: f64, t4514: f64, t51646: f64, t51653: f64, t51657: f64, t51660: f64, t51668: f64, t51672: f64, t51676: f64, t51680: f64, t51682: f64, t62385: f64, t62612: f64, t820: f64, t837: f64, t879: f64) -> f64 {
    let t63015 = t10542 * t18726;
    let t63024 = -0.65854491829355115987e0_f64 * t820 * t879 * t62385 - 0.92526556154787137113e-2_f64 * t51646 + 0.21951497276451705328e-1_f64 * t51653 - 0.1040793657534163522e0_f64 * t51657 + 0.39274398764404314548e-3_f64 * t51660 - 0.11708928647259339623e0_f64 * t51668 + 0.10975748638225852664e-1_f64 * t51672 - 0.39274398764404314548e-3_f64 * t51676 + 0.21951497276451705328e-1_f64 * t51680 - 0.19514881078765566038e-1_f64 * t63015 - 0.13170898365871023197e1_f64 * t4514 * t62612 * t837 - 0.23707617058567841754e2_f64 * t14546 * t18677 * t14547 + 0.2601984143835408805e-2_f64 * t51682;
    t63024
}
