//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3243/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3243<F: Float>(t33: F, t265: F, t502: F, t51814: F, t56291: F, t60124: F, t60130: F, t60139: F, t60142: F, t60143: F, t60147: F, t60155: F, t10326: F, t11095: F, t1113: F, t1304: F, t13196: F, t13312: F, t1469: F, t15083: F, t1587: F, t1711: F, t18140: F, t1837: F, t2258: F, t3351: F, t3805: F, t4186: F, t4560: F, t49889: F, t504: F, t51827: F, t51829: F, t51831: F, t51833: F, t51835: F, t5509: F, t57: F, t606: F, t9357: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t60159 = piecewise3::<F>(t503, t56291 + t60124 + t60130 + t60139 + t60142 + t60143 + t60147 + t60155, t51814);
    let t60177 = piecewise3::<F>(t400, t51814 * t33 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t15083 * t1113 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t4560 * t3351 + t1587 * t9357 / F::cast_from(2.0_f64) + t11095 * t1711 / F::cast_from(2.0_f64) - t51827 - t51829 + t51831 + t51833 - t51835, t60159 * t57 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t18140 * t606 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t5509 * t2258 - t1837 * t10326 / F::cast_from(2.0_f64) - t13196 * t1469 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3805 * t4186 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1304 * t13312 - t504 * t49889 / F::cast_from(2.0_f64));
    t60177
}
