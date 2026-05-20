//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2299/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2299<F: Float>(t33: F, t265: F, t502: F, t15083: F, t18127: F, t18138: F, t1113: F, t1304: F, t13312: F, t1469: F, t15093: F, t15094: F, t15096: F, t1587: F, t1711: F, t1837: F, t2258: F, t2838: F, t3351: F, t3805: F, t4186: F, t4560: F, t504: F, t5509: F, t57: F, t606: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t18140 = piecewise3::<F>(t503, t18127 + t18138, t15083);
    let t18152 = piecewise3::<F>(t400, t15083 * t33 / F::new(2.0) + t4560 * t1113 + t1587 * t3351 / F::new(2.0) + t2838 * t1711 / F::new(2.0) - t15093 - t15094 + t15096, t18140 * t57 / F::new(2.0) - t5509 * t606 - t1837 * t2258 / F::new(2.0) - t3805 * t1469 / F::new(2.0) - t1304 * t4186 - t504 * t13312 / F::new(2.0));
    (t18140, t18152)
}
