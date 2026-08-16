//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1542;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta374<F: Float>(t13969: F, t4599: F, t3039: F, t376: F, t4649: F, t4594: F, t4582: F, t3120: F, t3131: F, t4593: F, t10482: F, t3040: F, t3132: F, t3069: F, t4669: F, t10231: F, t4338: F, t973: F, t13542: F, t977: F, t10388: F, t10424: F, t10480: F, t10876: F, t10898: F, t10949: F, t13959: F, t13963: F, t13966: F, t1618: F, t3073: F, t3109: F, t3130: F, t4596: F, t4652: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13970, t13972, t13975, t13977, t13980, t13982, t13985) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1542::<F>(t13969, t4599, t3039, t376, t4649, t4594, t4582, t3120, t3131, t4593, t10482, t3040);
        let (t13987, t13991, t14004) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1543::<F>(t13985, t4593, t4582, t3132, t3069, t4669, t10231, t4338, t973, t13542, t977, t10388, t10424, t10480, t10876, t10898, t10949, t13959, t13963, t13966, t13972, t13977, t13982, t1618, t3073, t3109, t3130, t4596, t4652);
    (t13970, t13975, t13977, t13980, t13982, t13985, t13987, t13991, t14004)
}
