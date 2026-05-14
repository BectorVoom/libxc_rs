//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1183/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1183<F: Float>(t11116: F, t22274: F, t11069: F, t5662: F, t11016: F, t8478: F, t8638: F, t29052: F, t3025: F, t2714: F, t8556: F, t3040: F, t7593: F, t7596: F, t7590: F, t16251: F, t2103: F, t3447: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33851 = 0.1853729108614466568e0 * t22274 * t11116;
    let t33853 = 0.1022478025437886658e1 * t5662 * t11069;
    let t33857 = 0.14300195980740170668e1 * t8478 * t11016;
    let t33859 = 0.14300195980740170668e1 * t8638 * t11016;
    let t33861 = 0.14300195980740170668e1 * t3025 * t29052;
    let t33863 = 0.47667319935800568892e0 * t2714 * t8556;
    let t33865 = 0.35750489951850426669e0 * t7593 * t3040;
    let t33867 = 0.71500979903700853338e0 * t7596 * t3040;
    let t33869 = 0.35750489951850426669e0 * t7590 * t3040;
    let t33872 = 0.15889106645266856297e0 * t2103 * t16251 * t3447;
    (t33851, t33853, t33857, t33859, t33861, t33863, t33865, t33867, t33869, t33872)
}
