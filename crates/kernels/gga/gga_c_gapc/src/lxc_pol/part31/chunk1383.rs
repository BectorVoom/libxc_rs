//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1383/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1383<F: Float>(t33917: F, t33923: F, t33928: F, t33930: F, t33932: F, t33937: F, t33939: F, t33941: F, t33943: F, t33946: F, t33949: F, t33952: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36756 = F::new(0.47427337336674955566e-9) * t33917;
    let t36758 = F::new(0.69504740211613770836e-3) * t33923;
    let t36761 = F::new(0.22509399720615334744e-7) * t33928;
    let t36762 = F::new(0.1011909669415296852e-6) * t33930;
    let t36763 = F::new(0.11372686522837130914e-4) * t33932;
    let t36765 = F::new(0.22509399720615334744e-7) * t33937;
    let t36766 = F::new(0.22509399720615334744e-6) * t33939;
    let t36767 = F::new(0.33147827249531850013e-7) * t33941;
    let t36768 = F::new(0.66295654499063700026e-7) * t33943;
    let t36769 = F::new(0.33147827249531850013e-7) * t33946;
    let t36770 = F::new(0.13913017666225690434e-3) * t33949;
    let t36771 = F::new(0.69504740211613770836e-3) * t33952;
    (t36756, t36758, t36761, t36762, t36763, t36765, t36766, t36767, t36768, t36769, t36770, t36771)
}
