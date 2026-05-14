//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 518/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk518<F: Float>(t2792: F, t492: F, t105: F, t1063: F, t1358: F, t2268: F, t2308: F, t2313: F, t2319: F, t2323: F, t2328: F, t2738: F, t2741: F, t2757: F, t2762: F, t2766: F, t2780: F, t2784: F, t2789: F, t380: F, t419: F, t989: F, t994: F) -> (F, F) {
    let t2793 = t492 * t2792;
    let t2796 = 0.37940008847568199465e-1 * t380 * t989 + 0.28455006635676149599e-1 * t419 * t989 - 0.28455006635676149599e-1 * t1063 * t2738 + 0.28455006635676149599e-1 * t2268 * t2741 + 0.28455006635676149599e-1 * t105 * t2757 - 0.31616674039640166221e-2 * t1358 * t2762 - 0.85365019907028448797e-1 * t2268 * t2766 - 0.31616674039640166221e-2 * t2308 + 0.23712505529730124666e-2 * t2313 - 0.23712505529730124666e-2 * t2319 + 0.23712505529730124666e-2 * t2323 - 0.23712505529730124666e-2 * t2328 - 0.37940008847568199465e-1 * t380 * t994 - 0.28455006635676149599e-1 * t419 * t994 + 0.28455006635676149599e-1 * t1063 * t2780 + 0.31616674039640166221e-2 * t1358 * t2784 + 0.56910013271352299198e-1 * t2268 * t2789 - 0.28455006635676149599e-1 * t105 * t2793;
    (t2793, t2796)
}
