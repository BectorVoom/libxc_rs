//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1173/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1173<F: Float>(t33878: F, t33881: F, t33885: F, t33888: F, t33897: F, t33899: F, t33902: F, t33904: F, t33908: F, t33911: F, t33914: F, t33917: F, t33923: F, t33893: F, t33920: F, t33928: F) -> (F, F, F, F, F, F) {
    let t36743 = 0.20240885416666666668e-4 * t33878;
    let t36744 = 0.57920616843011475696e-5 * t33881;
    let t36745 = 0.8446756622939173539e-6 * t33885;
    let t36746 = 0.13493923611111111112e-4 * t33888;
    let t36749 = 0.58364997692245511715e-8 * t33897;
    let t36750 = 0.21102562238076876322e-7 * t33899;
    let t36751 = 0.2748593934505475288e-6 * t33902;
    let t36752 = 0.36652500116630512966e-6 * t33904;
    let t36753 = 0.13493923611111111112e-4 * t33908;
    let t36754 = 0.20240885416666666668e-3 * t33911;
    let t36755 = 0.12290803273518880209e-8 * t33914;
    let t36756 = 0.47427337336674955566e-9 * t33917;
    let t36758 = 0.69504740211613770836e-3 * t33923;
    let t36759 = 0.24598298249421296299e-6 * t33893 - t36749 - t36750 + t36751 - t36752 + t36753 - t36754 + t36755 - t36756 + 0.505954834707648426e-7 * t33920 + t36758;
    let t36761 = 0.22509399720615334744e-7 * t33928;
    (t36743, t36744, t36745, t36746, t36759, t36761)
}
