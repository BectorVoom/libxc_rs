//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1172/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1172<F: Float>(t251: F, t5977: F, t1558: F, t1568: F, t10519: F, t10539: F, t14498: F, t14506: F, t14511: F, t14512: F, t14518: F, t14522: F, t14525: F, t14533: F, t14539: F, t2815: F, t4424: F, t4494: F, t4514: F, t5978: F, t820: F, t837: F) -> (F, F, F) {
    let t18677 = t251 * t5977;
    let t18681 = t1568 * t1558;
    let t18687 = F::cast_from(0.13009920719177044025e-1_f64) * t10519 + t14498 + t14506 + t14511 + F::cast_from(0.13009920719177044025e-2_f64) * t14512 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t2815 * t5978 - t14518 - t14522 - F::cast_from(0.23131639038696784278e-2_f64) * t14525 - F::cast_from(0.13170898365871023197e1_f64) * t4514 * t4494 * t4424 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t18677 * t837 - F::cast_from(0.13170898365871023197e1_f64) * t4514 * t18681 * t837 - F::cast_from(0.14634331517634470219e-1_f64) * t14533 + t14539 - F::cast_from(0.11565819519348392139e-2_f64) * t10539;
    (t18677, t18681, t18687)
}
