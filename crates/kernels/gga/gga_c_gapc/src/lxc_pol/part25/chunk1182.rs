//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1182/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1182<F: Float>(t34679: F, t34682: F, t34686: F, t34689: F, t34692: F, t34698: F, t34704: F, t34712: F, t34715: F, t34718: F, t34720: F, t34723: F, t34726: F, t34729: F, t34732: F, t34735: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37034 = 0.69504740211613770836e-3 * t34679;
    let t37035 = 0.35265860507710533408e-5 * t34682;
    let t37036 = 0.4637672555408563478e-4 * t34686;
    let t37037 = 0.21642471925239962898e-3 * t34689;
    let t37038 = 0.73295838253479341016e-5 * t34692;
    let t37040 = 0.47427337336674955566e-9 * t34698;
    let t37042 = 0.2949982546997070313e-7 * t34704;
    let t37046 = 0.11272120794395814009e-6 * t34712;
    let t37047 = 0.26681999992642267404e-7 * t34715;
    let t37048 = 0.2318836277704281739e-4 * t34718;
    let t37049 = 0.3243554543208642639e-2 * t34720;
    let t37050 = 0.69504740211613770836e-3 * t34723;
    let t37051 = 0.69504740211613770836e-3 * t34726;
    let t37052 = 0.34752370105806885418e-3 * t34729;
    let t37053 = 0.67402122125348062552e-7 * t34732;
    let t37054 = 0.11372686522837130914e-5 * t34735;
    (t37034, t37035, t37036, t37037, t37038, t37040, t37042, t37046, t37047, t37048, t37049, t37050, t37051, t37052, t37053, t37054)
}
