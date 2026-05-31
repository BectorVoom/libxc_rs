//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1470/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1470<F: Float>(t11399: F, t11411: F, t11444: F, t11445: F, t11450: F, t11466: F, t11468: F, t11501: F, t11507: F, t11513: F, t11548: F, t2938: F, t2943: F, t2945: F, t2962: F, t2963: F, t2968: F, t2970: F, t2971: F, t2987: F, t2988: F, t2989: F, t3006: F, t3012: F, t3014: F, t3015: F, t41225: F, t41513: F, t41668: F, t41779: F, t41785: F, t41788: F, t41794: F, t41799: F, t41813: F, t955: F, t974: F) -> F {
    let t41825 = -F::cast_from(0.77193501593724168322e3_f64) * t41779 * t11411 + F::cast_from(0.11579025239058625248e4_f64) * t11450 * t41668 * t2970 - F::cast_from(0.70178683471615754484e1_f64) * t41785 * t2989 - F::cast_from(0.4155806185363551302e3_f64) * t41788 * t11468 + F::cast_from(0.6233709278045326953e3_f64) * t11507 * t41225 * t3014 + F::cast_from(4.0_f64) * t41794 * t955 + F::cast_from(6.0_f64) * t11399 * t2963 + F::cast_from(0.1929837539843104208e3_f64) * t41799 * t2971 + F::cast_from(4.0_f64) * t2938 * t11445 + F::cast_from(0.21053605041484726346e2_f64) * t3012 * t2989 * t3006 - F::cast_from(0.46785788981077169656e1_f64) * t2987 * t974 * t11501 - F::cast_from(0.62337092780453269531e3_f64) * t11466 * t3015 * t3006 - t41513 + F::cast_from(0.61524113149298439947e4_f64) * t11507 * t41813 * t2988 - F::cast_from(24.0_f64) * t11548 * t11513 + F::cast_from(36.0_f64) * t2968 * t2945 * t2962 - F::cast_from(8.0_f64) * t2943 * t955 * t11444;
    t41825
}
