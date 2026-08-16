//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1384/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1384(t33908: f64, t33911: f64, t33914: f64, t33917: f64, t33923: f64, t33893: f64, t33920: f64, t36749: f64, t36750: f64, t36751: f64, t36752: f64, t33928: f64) -> (f64, f64) {
    let t36753 = 0.13493923611111111112e-4_f64 * t33908;
    let t36754 = 0.20240885416666666668e-3_f64 * t33911;
    let t36755 = 0.12290803273518880209e-8_f64 * t33914;
    let t36756 = 0.47427337336674955566e-9_f64 * t33917;
    let t36758 = 0.69504740211613770836e-3_f64 * t33923;
    let t36759 = 0.24598298249421296299e-6_f64 * t33893 - t36749 - t36750 + t36751 - t36752 + t36753 - t36754 + t36755 - t36756 + 0.505954834707648426e-7_f64 * t33920 + t36758;
    let t36761 = 0.22509399720615334744e-7_f64 * t33928;
    (t36759, t36761)
}
