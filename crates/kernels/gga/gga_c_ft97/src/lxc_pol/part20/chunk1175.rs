//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1175/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1175<F: Float>(t668: F, t880: F, t1477: F, t9568: F, t317: F, t9570: F, t13863: F, t14075: F, t14116: F, t15407: F, t15533: F, t2347: F, t2404: F, t25412: F, t25413: F, t25415: F, t25459: F, t28868: F, t28934: F, t28935: F, t28944: F, t28945: F, t28946: F, t28951: F, t29000: F, t29008: F, t3886: F, t4255: F, t6216: F, t6261: F, t684: F, t98694: F) -> (F,) {
    let t111592 = t880 * t668;
    let t111624 = t9568 * t1477;
    let t111625 = t317 * t9570;
    let t111636 = 2.0 / 9.0 * t25459 * t28935 + 2.0 / 9.0 * t6216 * t98694 * t28934 + 2.0 / 9.0 * t6216 * t25412 * t111592 * t4255 + t6216 * t25412 * t25413 * t15533 / 9.0 - 4.0 / 9.0 * t29000 * t25412 * t25413 * t15407 + 2.0 / 9.0 * t6216 * t25412 * t28868 * t684 + 2.0 / 9.0 * t29008 * t25415 - 2.0 / 27.0 * t6216 * t2404 * t6261 * t28946 - 2.0 / 27.0 * t6216 * t28944 * t880 * t2347 * t3886 - t6216 * t28944 * t28945 * t14075 / 27.0 - 5.0 / 81.0 * t6216 * t111624 * t111625 * t13863 + 4.0 / 27.0 * t29000 * t28944 * t28945 * t14116 + 2.0 / 9.0 * t25459 * t28951;
    (t111636,)
}
