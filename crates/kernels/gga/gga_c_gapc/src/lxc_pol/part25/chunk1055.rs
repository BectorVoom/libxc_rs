//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1055/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1055<F: Float>(t11816: F, t11818: F, t11820: F, t11823: F, t11829: F, t11832: F, t11838: F, t11843: F, t11845: F, t11851: F, t11855: F, t11863: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12219 = F::new(0.12290803273518880209e-8) * t11816;
    let t12220 = F::new(0.32042899674547455013e-6) * t11818;
    let t12221 = F::new(0.11254699860307667372e-6) * t11820;
    let t12222 = F::new(0.30353495895471971565e-6) * t11823;
    let t12224 = F::new(0.12290803273518880209e-8) * t11829;
    let t12225 = F::new(0.8193868849012586806e-9) * t11832;
    let t12226 = F::new(0.11049275749843950004e-7) * t11838;
    let t12228 = F::new(0.11594181388521408695e-4) * t11843;
    let t12229 = F::new(0.11594181388521408695e-4) * t11845;
    let t12230 = F::new(0.28960308421505737848e-5) * t11851;
    let t12231 = F::new(0.25340269868817520617e-3) * t11855;
    let t12235 = F::new(0.20240885416666666668e-4) * t11863;
    (t12219, t12220, t12221, t12222, t12224, t12225, t12226, t12228, t12229, t12230, t12231, t12235)
}
