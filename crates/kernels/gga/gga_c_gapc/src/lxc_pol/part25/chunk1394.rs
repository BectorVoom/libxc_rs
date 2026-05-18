//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1394/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1394<F: Float>(t34666: F, t34669: F, t34673: F, t34676: F, t34679: F, t34682: F, t34686: F, t34689: F, t34692: F, t34698: F, t34704: F, t34712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37029 = F::new(0.54924190264999682021e-4) * t34666;
    let t37030 = F::new(0.4637672555408563478e-4) * t34669;
    let t37032 = F::new(0.69504740211613770836e-3) * t34673;
    let t37033 = F::new(0.9275345110817126956e-4) * t34676;
    let t37034 = F::new(0.69504740211613770836e-3) * t34679;
    let t37035 = F::new(0.35265860507710533408e-5) * t34682;
    let t37036 = F::new(0.4637672555408563478e-4) * t34686;
    let t37037 = F::new(0.21642471925239962898e-3) * t34689;
    let t37038 = F::new(0.73295838253479341016e-5) * t34692;
    let t37040 = F::new(0.47427337336674955566e-9) * t34698;
    let t37042 = F::new(0.2949982546997070313e-7) * t34704;
    let t37046 = F::new(0.11272120794395814009e-6) * t34712;
    (t37029, t37030, t37032, t37033, t37034, t37035, t37036, t37037, t37038, t37040, t37042, t37046)
}
