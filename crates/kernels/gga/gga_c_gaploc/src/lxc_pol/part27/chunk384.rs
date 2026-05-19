//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 384/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk384<F: Float>(t109: F, t111: F, t1275: F, t1279: F, t1286: F, t1735: F, t1742: F, t1763: F, t1767: F, t1832: F, t260: F, t271: F, t427: F, t436: F, t437: F, t695: F) -> F {
    let t1835 = -F::cast_from(0.11281315546296296296e-3_f64) * t109 * t1275 * t271 + F::new(0.1e-22) * t436 * t1279 * t271 - F::cast_from(0.67687893277777777778e-3_f64) * t109 * t427 * t695 + F::cast_from(0.50765919958333333334e-3_f64) * t1286 * t1735 + F::cast_from(0.50765919958333333334e-3_f64) * t436 * t437 * t695 + F::cast_from(0.10153183991666666667e-2_f64) * t109 * t111 * t1742 - F::cast_from(0.50765919958333333334e-3_f64) * t109 * t111 * t1763 - F::new(4.0) * t1767 - F::new(4.0) * t260 * t1832;
    t1835
}
