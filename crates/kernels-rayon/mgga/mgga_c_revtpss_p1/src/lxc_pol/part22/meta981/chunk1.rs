//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3312/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3312(t10657: f64, t14663: f64, t14972: f64, t18714: f64, t2646: f64, t39656: f64, t39731: f64, t4424: f64, t4514: f64, t4526: f64, t51470: f64, t51483: f64, t6017: f64, t6022: f64, t62760: f64, t62763: f64, t62775: f64, t62777: f64, t62788: f64, t820: f64, t837: f64) -> f64 {
    let t62792 = -0.29268663035268940438e-1_f64 * t51470 - 0.13170898365871023197e1_f64 * t820 * t4526 * t14663 + 0.10975748638225852664e-1_f64 * t62763 - 0.13170898365871023197e1_f64 * t4514 * t62760 * t837 - 0.65854491829355115987e0_f64 * t820 * t10657 * t6017 - 0.46263278077393568556e-2_f64 * t51483 + 0.2601984143835408805e-1_f64 * t39731 - 0.10975748638225852664e-1_f64 * t62775 + 0.14634331517634470219e-1_f64 * t62777 - 0.26341796731742046394e1_f64 * t820 * t14972 * t4424 - 0.65854491829355115987e0_f64 * t820 * t18714 * t2646 + 0.13170898365871023197e1_f64 * t820 * t39656 * t6022 - 0.13170898365871023197e1_f64 * t820 * t62788 * t837;
    t62792
}
