//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1205/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1205<F: Float>(t1568: F, t2718: F, t4469: F, t822: F, t10923: F, t10925: F, t10930: F, t10935: F, t10939: F, t10948: F, t10961: F, t10964: F, t10966: F, t10969: F, t10971: F, t10974: F, t14507: F, t2646: F, t2724: F, t4514: F, t4526: F, t820: F, t837: F) -> F {
    let t14961 = t2718 * t1568;
    let t14972 = t822 * t4469;
    let t14976 = -F::cast_from(0.14634331517634470219e-1_f64) * t10923 + F::cast_from(0.13009920719177044025e-2_f64) * t10925 + F::cast_from(0.10975748638225852664e-1_f64) * t10930 + F::cast_from(0.54878743191129263322e-2_f64) * t10935 - F::cast_from(0.13170898365871023197e1_f64) * t4514 * t14507 * t837 + t10939 + F::cast_from(0.13170898365871023197e1_f64) * t820 * t14961 * t2724 - t10948 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t4526 * t2646 - F::cast_from(0.54878743191129263322e-2_f64) * t10961 - F::cast_from(0.13009920719177044025e-2_f64) * t10964 + F::cast_from(0.14634331517634470219e-1_f64) * t10966 + t10969 - t10971 + F::cast_from(0.9757440539382783019e-2_f64) * t10974 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t14972 * t837;
    t14976
}
