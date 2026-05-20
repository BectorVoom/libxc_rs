//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1723/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1723<F: Float>(t30: F, t265: F, t393: F, t87990: F, t88042: F, t88577: F, t88603: F, t89756: F, t1468: F, t1469: F, t1587: F, t1704: F, t22670: F, t22671: F, t23436: F, t24192: F, t395: F, t45: F, t5824: F, t5825: F, t6084: F, t6405: F, t87125: F, t87126: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t89759 = piecewise3::<F>(t394, t88042 + t88577 + t88603 + t89756, t87990);
    let t89771 = piecewise3::<F>(t120, t87990 * t30 / F::new(2.0) + F::new(2.0) * t23436 * t1468 + F::new(3.0) * t6084 * t5824 + F::new(2.0) * t1587 * t22670 + t265 * t87125 / F::new(2.0), t89759 * t45 / F::new(2.0) + F::new(2.0) * t24192 * t1469 + F::new(3.0) * t6405 * t5825 + F::new(2.0) * t1704 * t22671 + t395 * t87126 / F::new(2.0));
    t89771
}
