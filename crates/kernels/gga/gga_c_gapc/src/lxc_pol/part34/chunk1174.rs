//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1174/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1174<F: Float>(t33930: F, t33932: F, t33937: F, t33939: F, t33941: F, t33943: F, t33946: F, t33949: F, t33952: F, t33935: F, t36761: F, t33956: F, t33962: F, t33967: F, t33972: F, t33975: F) -> (F, F, F, F, F, F) {
    let t36762 = 0.1011909669415296852e-6 * t33930;
    let t36763 = 0.11372686522837130914e-4 * t33932;
    let t36765 = 0.22509399720615334744e-7 * t33937;
    let t36766 = 0.22509399720615334744e-6 * t33939;
    let t36767 = 0.33147827249531850013e-7 * t33941;
    let t36768 = 0.66295654499063700026e-7 * t33943;
    let t36769 = 0.33147827249531850013e-7 * t33946;
    let t36770 = 0.13913017666225690434e-3 * t33949;
    let t36771 = 0.69504740211613770836e-3 * t33952;
    let t36772 = t36761 - t36762 + t36763 + 0.25301106770833333336e-5 * t33935 + t36765 + t36766 + t36767 + t36768 + t36769 - t36770 - t36771;
    let t36773 = 0.67402122125348062552e-7 * t33956;
    let t36774 = 0.20041830772435757309e-6 * t33962;
    let t36775 = 0.83645744500336823644e-8 * t33967;
    let t36777 = 0.2318836277704281739e-4 * t33972;
    let t36778 = 0.71696352428860134552e-9 * t33975;
    (t36772, t36773, t36774, t36775, t36777, t36778)
}
