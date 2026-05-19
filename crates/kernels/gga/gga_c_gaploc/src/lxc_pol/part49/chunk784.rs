//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 784/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk784<F: Float>(t13749: F, t493: F, t492: F, t105: F, t169: F, t172: F, t452: F, t12771: F, t12799: F, t12805: F, t12812: F, t12821: F, t12823: F, t12824: F, t12825: F, t12828: F, t12829: F, t12832: F, t12833: F) -> (F, F, F, F, F) {
    let t13750 = t493 * t13749;
    let t13751 = t492 * t13750;
    let t13753 = F::cast_from(0.28455006635676149599e-1_f64) * t105 * t13751;
    let t13755 = t13749 * t169 * t172;
    let t13756 = t452 * t13755;
    let t13758 = F::cast_from(0.28455006635676149599e-1_f64) * t105 * t13756;
    let t13759 = t12812 + t12828 + F::cast_from(0.11856252764865062333e-2_f64) * t12771 - F::cast_from(0.11856252764865062333e-2_f64) * t12821 - t13753 + t13758 + t12829 - t12833 - t12823 + t12824 + t12825 + t12799 + t12805 - t12832;
    (t13750, t13751, t13755, t13756, t13759)
}
