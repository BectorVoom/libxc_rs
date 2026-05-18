//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1381/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1381<F: Float>(t33704: F, t33707: F, t33710: F, t33714: F, t33717: F, t33726: F, t33728: F, t33731: F, t33734: F, t33719: F, t36671: F, t33741: F) -> (F, F) {
    let t36672 = F::new(0.43440462632258606772e-4) * t33704;
    let t36673 = F::new(0.21720231316129303386e-4) * t33707;
    let t36674 = F::new(0.41223756048076119805e-5) * t33710;
    let t36675 = F::new(0.73295838253479341016e-5) * t33714;
    let t36676 = F::new(0.73744819641113281254e-8) * t33717;
    let t36678 = F::new(0.40481770833333333336e-4) * t33726;
    let t36679 = F::new(0.11372686522837130914e-5) * t33728;
    let t36680 = F::new(0.11372686522837130914e-5) * t33731;
    let t36681 = F::new(0.4637672555408563478e-4) * t33734;
    let t36682 = -t36671 - t36672 - t36673 - t36674 + t36675 - t36676 + F::new(0.12650553385416666668e-5) * t33719 + t36678 - t36679 - t36680 + t36681;
    let t36687 = F::new(0.43284943850479925795e-3) * t33741;
    (t36682, t36687)
}
