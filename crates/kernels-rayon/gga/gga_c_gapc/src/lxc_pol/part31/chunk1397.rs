//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1397/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1397(t34666: f64, t34669: f64, t34673: f64, t34676: f64, t34679: f64, t34682: f64, t34686: f64, t34689: f64, t34692: f64, t34698: f64, t34704: f64, t34712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37029 = 0.54924190264999682021e-4_f64 * t34666;
    let t37030 = 0.4637672555408563478e-4_f64 * t34669;
    let t37032 = 0.69504740211613770836e-3_f64 * t34673;
    let t37033 = 0.9275345110817126956e-4_f64 * t34676;
    let t37034 = 0.69504740211613770836e-3_f64 * t34679;
    let t37035 = 0.35265860507710533408e-5_f64 * t34682;
    let t37036 = 0.4637672555408563478e-4_f64 * t34686;
    let t37037 = 0.21642471925239962898e-3_f64 * t34689;
    let t37038 = 0.73295838253479341016e-5_f64 * t34692;
    let t37040 = 0.47427337336674955566e-9_f64 * t34698;
    let t37042 = 0.2949982546997070313e-7_f64 * t34704;
    let t37046 = 0.11272120794395814009e-6_f64 * t34712;
    (t37029, t37030, t37032, t37033, t37034, t37035, t37036, t37037, t37038, t37040, t37042, t37046)
}
