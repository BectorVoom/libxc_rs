//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1384/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1384<F: Float>(t33908: F, t33911: F, t33914: F, t33917: F, t33923: F, t33893: F, t33920: F, t36749: F, t36750: F, t36751: F, t36752: F, t33928: F) -> (F, F) {
    let t36753 = F::new(0.13493923611111111112e-4) * t33908;
    let t36754 = F::new(0.20240885416666666668e-3) * t33911;
    let t36755 = F::new(0.12290803273518880209e-8) * t33914;
    let t36756 = F::new(0.47427337336674955566e-9) * t33917;
    let t36758 = F::new(0.69504740211613770836e-3) * t33923;
    let t36759 = F::new(0.24598298249421296299e-6) * t33893 - t36749 - t36750 + t36751 - t36752 + t36753 - t36754 + t36755 - t36756 + F::new(0.505954834707648426e-7) * t33920 + t36758;
    let t36761 = F::new(0.22509399720615334744e-7) * t33928;
    (t36759, t36761)
}
