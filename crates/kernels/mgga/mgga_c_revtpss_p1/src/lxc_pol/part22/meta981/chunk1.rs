//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3312/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3312<F: Float>(t10657: F, t14663: F, t14972: F, t18714: F, t2646: F, t39656: F, t39731: F, t4424: F, t4514: F, t4526: F, t51470: F, t51483: F, t6017: F, t6022: F, t62760: F, t62763: F, t62775: F, t62777: F, t62788: F, t820: F, t837: F) -> F {
    let t62792 = -F::cast_from(0.29268663035268940438e-1_f64) * t51470 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t4526 * t14663 + F::cast_from(0.10975748638225852664e-1_f64) * t62763 - F::cast_from(0.13170898365871023197e1_f64) * t4514 * t62760 * t837 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t10657 * t6017 - F::cast_from(0.46263278077393568556e-2_f64) * t51483 + F::cast_from(0.2601984143835408805e-1_f64) * t39731 - F::cast_from(0.10975748638225852664e-1_f64) * t62775 + F::cast_from(0.14634331517634470219e-1_f64) * t62777 - F::cast_from(0.26341796731742046394e1_f64) * t820 * t14972 * t4424 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t18714 * t2646 + F::cast_from(0.13170898365871023197e1_f64) * t820 * t39656 * t6022 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t62788 * t837;
    t62792
}
