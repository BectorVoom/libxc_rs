//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1325/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1325<F: Float>(t33694: F, t11050: F, t1986: F, t28793: F, t28796: F, t28800: F, t28810: F, t33666: F, t33668: F, t33671: F, t33673: F, t33675: F, t33676: F, t33683: F, t33685: F, t33690: F, t33692: F, t5662: F, t590: F) -> F {
    let t33695 = F::cast_from(0.29792074959875355558e-1_f64) * t33694;
    let t33696 = t33666 + t33668 + t33671 - t33673 - t33675 - F::cast_from(0.1022478025437886658e1_f64) * t1986 * t33676 * t590 + t33683 - t33685 + t28793 + t28796 + t28800 - F::cast_from(0.51123901271894332905e0_f64) * t5662 * t11050 - t28810 - t33690 + t33692 - t33695;
    t33696
}
