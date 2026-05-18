//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1317/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1317<F: Float>(t30: F, t265: F, t393: F, t15083: F, t15546: F, t16616: F, t1106: F, t13312: F, t1468: F, t1469: F, t15093: F, t15094: F, t15096: F, t1587: F, t1704: F, t2257: F, t2258: F, t2838: F, t3340: F, t395: F, t4186: F, t45: F, t4560: F, t5028: F, t605: F, t606: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t16618 = piecewise3::<f64>(t394, t15546 + t16616, t15083);
    let t16630 = piecewise3::<f64>(t120, t15083 * t30 / F::new(2.0) + t4560 * t605 + t1587 * t2257 / F::new(2.0) + t2838 * t1468 / F::new(2.0) + t15093 + t15094 - t15096, t16618 * t45 / F::new(2.0) + t5028 * t606 + t1704 * t2258 / F::new(2.0) + t3340 * t1469 / F::new(2.0) + t1106 * t4186 + t395 * t13312 / F::new(2.0));
    t16630
}
