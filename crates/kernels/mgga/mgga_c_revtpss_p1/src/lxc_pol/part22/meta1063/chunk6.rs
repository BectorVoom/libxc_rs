//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3810/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3810<F: Float>(t33: F, t265: F, t502: F, t63193: F, t68629: F, t73260: F, t73266: F, t73270: F, t73277: F, t73283: F, t73285: F, t73286: F, t1113: F, t1304: F, t13312: F, t1469: F, t15083: F, t1711: F, t18140: F, t18281: F, t1837: F, t18884: F, t20256: F, t21645: F, t2258: F, t2838: F, t3351: F, t3805: F, t4186: F, t504: F, t51835: F, t5509: F, t57: F, t5825: F, t606: F, t60754: F, t6084: F, t63202: F, t63204: F, t63206: F, t6416: F, t6757: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t73290 = piecewise3::<F>(t503, t68629 + t73260 + t73266 + t73270 + t73277 + t73283 + t73285 + t73286, t63193);
    let t73306 = piecewise3::<F>(t400, t63193 * t33 / F::new(2.0) + t18884 * t1113 + t6084 * t3351 / F::new(2.0) + t15083 * t1711 - t63202 - t63204 + t63206 + t2838 * t6416 / F::new(2.0) + t895 * t20256 - t51835, t73290 * t57 / F::new(2.0) - t21645 * t606 - t6757 * t2258 / F::new(2.0) - t18140 * t1469 - F::new(2.0) * t5509 * t4186 - t1837 * t13312 - t3805 * t5825 / F::new(2.0) - t1304 * t18281 - t504 * t60754 / F::new(2.0));
    t73306
}
