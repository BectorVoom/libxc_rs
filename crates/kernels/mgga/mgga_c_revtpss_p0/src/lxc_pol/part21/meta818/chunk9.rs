//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3020/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3020<F: Float>(t11951: F, t4858: F, t11922: F, t15906: F, t15909: F, t16067: F, t16069: F, t11200: F, t380: F, t16088: F, t1025: F, t11623: F, t11783: F, t15651: F, t15717: F, t15780: F, t15785: F, t15895: F, t16017: F, t16049: F, t3092: F, t3117: F, t3224: F, t371: F, t372: F, t373: F, t42765: F, t4854: F, t4879: F, t4892: F, t53273: F, t906: F) -> (F, F) {
    let t55320 = t4858 * t11951;
    let t55325 = t15906 * t11922 * t15909;
    let t55328 = t16067 * t11922 * t16069;
    let t55330 = t11200 * t380;
    let t55331 = t55330 * t16088;
    let t55338 = F::cast_from(0.12862205435420921092e-2_f64) * t4892 * t3117 * t15780 * t15785 + F::cast_from(0.21437009059034868486e-3_f64) * t4879 * t11623 - F::cast_from(0.64311027177104605458e-3_f64) * t11783 * t4854 - F::cast_from(0.64311027177104605458e-3_f64) * t3224 * t15651 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t371 * t372 * t373 * t53273 - F::cast_from(0.42874018118069736972e-3_f64) * t55320 + F::cast_from(0.68598428988911579154e-2_f64) * t16049 * t16017 - F::cast_from(0.25724410870841842183e-2_f64) * t55325 + F::cast_from(0.42874018118069736972e-3_f64) * t55328 - F::cast_from(0.25724410870841842184e-2_f64) * t55331 * t3092 * t15717 * t906 + F::cast_from(0.68598428988911579154e-2_f64) * t42765 * t15895;
    (t55330, t55338)
}
