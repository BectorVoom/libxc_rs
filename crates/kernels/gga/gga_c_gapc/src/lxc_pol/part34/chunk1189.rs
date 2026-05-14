//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1189/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1189<F: Float>(t34689: F, t34692: F, t34698: F, t34704: F, t34695: F, t34701: F, t37032: F, t37033: F, t37034: F, t37035: F, t37036: F, t34712: F, t34715: F, t34718: F, t34720: F, t34723: F) -> (F, F, F, F, F, F) {
    let t37037 = 0.21642471925239962898e-3 * t34689;
    let t37038 = 0.73295838253479341016e-5 * t34692;
    let t37040 = 0.47427337336674955566e-9 * t34698;
    let t37042 = 0.2949982546997070313e-7 * t34704;
    let t37043 = t37032 - t37033 + t37034 - t37035 - t37036 - t37037 + t37038 + 0.88397049170382309318e-7 * t34695 - t37040 + 0.66297786877786731988e-7 * t34701 - t37042;
    let t37046 = 0.11272120794395814009e-6 * t34712;
    let t37047 = 0.26681999992642267404e-7 * t34715;
    let t37048 = 0.2318836277704281739e-4 * t34718;
    let t37049 = 0.3243554543208642639e-2 * t34720;
    let t37050 = 0.69504740211613770836e-3 * t34723;
    (t37043, t37046, t37047, t37048, t37049, t37050)
}
