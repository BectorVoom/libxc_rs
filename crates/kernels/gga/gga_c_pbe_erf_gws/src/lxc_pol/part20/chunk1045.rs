//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1045/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1045<F: Float>(t14605: F, t14611: F, t14655: F, t14689: F, t14708: F, t14716: F, t14745: F, t14752: F, t14506: F, t14520: F, t14551: F, t14554: F, t14558: F, t14563: F, t3703: F, t3944: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14898 = 7.0 / 2304.0 * t14605;
    let t14931 = 7.0 / 2304.0 * t14611;
    let t14962 = 7.0 / 576.0 * t14655;
    let t14974 = 7.0 / 144.0 * t14689;
    let t14978 = 7.0 / 144.0 * t14708;
    let t14986 = 7.0 / 1152.0 * t14716;
    let t14996 = 7.0 / 72.0 * t14745;
    let t14999 = 7.0 / 144.0 * t14752;
    let t15050 = 7.0 / 576.0 * t14506;
    let t15057 = 7.0 / 144.0 * t14520;
    let t15070 = 7.0 / 576.0 * t14551;
    let t15072 = 7.0 / 144.0 * t14554;
    let t15074 = 7.0 / 288.0 * t14558;
    let t15076 = 7.0 / 72.0 * t14563;
    let t15118 = t3944 * t3703;
    (t14898, t14931, t14962, t14974, t14978, t14986, t14996, t14999, t15050, t15057, t15070, t15072, t15074, t15076, t15118)
}
