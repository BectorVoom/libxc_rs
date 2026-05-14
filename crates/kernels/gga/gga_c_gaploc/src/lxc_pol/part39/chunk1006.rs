//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1006/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1006<F: Float>(t38770: F, t901: F, t38486: F, t13792: F, t4379: F, t12000: F, t1429: F, t2365: F, t2366: F, t47953: F, t6963: F, t6964: F, t13801: F, t1641: F, t41960: F, t41962: F, t41968: F, t41970: F, t41972: F, t41973: F) -> (F,) {
    let t47976 = t38770 * t901;
    let t47978 = t38486 * t901;
    let t47980 = t4379 * t13792;
    let t47984 = t1429 * t2365 * t2366 * t12000;
    let t47987 = t6963 * t6964 * t47953;
    let t47989 = t1641 * t13801;
    let t47992 = 0.14896037479937677779e-1 * t41960 + 0.14896037479937677779e-1 * t41962 + 0.14896037479937677779e-1 * t47976 + 0.14896037479937677779e-1 * t47978 - 0.14896037479937677779e-1 * t47980 - 0.14896037479937677779e-1 * t47984 - 0.71500979903700853338e0 * t47987 - 0.46011511144704899612e1 * t47989 + t41968 + 0.46011511144704899612e1 * t41970 - t41972 - t41973;
    (t47992,)
}
