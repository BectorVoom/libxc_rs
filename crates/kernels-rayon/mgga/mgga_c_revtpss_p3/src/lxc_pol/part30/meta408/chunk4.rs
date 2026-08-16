//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1528/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1528(t1568: f64, t2718: f64, t4469: f64, t822: f64, t10923: f64, t10925: f64, t10930: f64, t10935: f64, t10939: f64, t10948: f64, t10961: f64, t10964: f64, t10966: f64, t10969: f64, t10971: f64, t10974: f64, t14507: f64, t2646: f64, t2724: f64, t4514: f64, t4526: f64, t820: f64, t837: f64) -> f64 {
    let t14961 = t2718 * t1568;
    let t14972 = t822 * t4469;
    let t14976 = -0.14634331517634470219e-1_f64 * t10923 + 0.13009920719177044025e-2_f64 * t10925 + 0.10975748638225852664e-1_f64 * t10930 + 0.54878743191129263322e-2_f64 * t10935 - 0.13170898365871023197e1_f64 * t4514 * t14507 * t837 + t10939 + 0.13170898365871023197e1_f64 * t820 * t14961 * t2724 - t10948 - 0.65854491829355115987e0_f64 * t820 * t4526 * t2646 - 0.54878743191129263322e-2_f64 * t10961 - 0.13009920719177044025e-2_f64 * t10964 + 0.14634331517634470219e-1_f64 * t10966 + t10969 - t10971 + 0.9757440539382783019e-2_f64 * t10974 - 0.13170898365871023197e1_f64 * t820 * t14972 * t837;
    t14976
}
