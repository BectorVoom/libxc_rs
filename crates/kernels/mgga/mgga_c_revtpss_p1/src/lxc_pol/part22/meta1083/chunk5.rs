//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3918/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3918<F: Float>(t22394: F, t686: F, t72: F, t9680: F, t10171: F, t13747: F, t1424: F, t14269: F, t1427: F, t4076: F, t47595: F, t47601: F, t47608: F, t47616: F, t47618: F, t47620: F, t49512: F, t49522: F, t49528: F, t5715: F, t6896: F, t74836: F, t74838: F, t74843: F, t74849: F, t74853: F, t74855: F, t74890: F, t74926: F, t74954: F, t74987: F, t75009: F, t75044: F, t75070: F, t75097: F, t75125: F, t75155: F, t75182: F, t75209: F, t75242: F, t75263: F, t75295: F, t75324: F) -> F {
    let t75336 = t9680 * t22394 * t72 * t686;
    let t75343 = -F::cast_from(0.73171657588172351096e-2_f64) * t47595 + t47601 + F::cast_from(0.52039682876708176102e-1_f64) * t49512 - F::cast_from(0.19514881078765566038e-1_f64) * t74836 + F::cast_from(0.26019841438354088049e-1_f64) * t74838 - F::cast_from(0.21951497276451705328e-1_f64) * t49522 - F::cast_from(0.11708928647259339622e0_f64) * t74843 + F::cast_from(0.14634331517634470219e-1_f64) * t47608 - F::cast_from(0.13170898365871023197e1_f64) * t5715 * t14269 - F::cast_from(0.19514881078765566038e-1_f64) * t49528 - F::cast_from(0.14634331517634470219e-1_f64) * t74849 - F::cast_from(0.10975748638225852664e-1_f64) * t74853 + F::cast_from(0.26341796731742046394e1_f64) * t1424 * t4076 * t74855 - F::cast_from(0.65854491829355115987e0_f64) * t1424 * t1427 * (t74890 + t74926 + t74954 + t74987 + t75009 + t75044 + t75070 + t75097 + t75125 + t75155 + t75182 + t75209 + t75242 + t75263 + t75295 + t75324) + F::cast_from(0.52683593463484092788e1_f64) * t5715 * t13747 + F::cast_from(0.78059524315062264149e-1_f64) * t75336 - F::cast_from(0.65049603595885220126e-3_f64) * t47616 + F::cast_from(0.52039682876708176102e-2_f64) * t47618 + F::cast_from(0.73171657588172351096e-2_f64) * t47620 + F::cast_from(0.13170898365871023197e1_f64) * t10171 * t6896;
    t75343
}
