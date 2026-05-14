//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1412/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1412<F: Float>(t10297: F, t10300: F, t10305: F, t10361: F, t10362: F, t1246: F, t1256: F, t12845: F, t158: F, t23398: F, t2428: F, t2430: F, t2453: F, t28400: F, t3247: F, t3254: F, t3255: F, t3278: F, t3904: F, t411: F, t415: F, t8481: F, t8504: F, t8559: F, t8560: F, t938: F, t951: F) -> (F,) {
    let t28569 = 0.52683593463484092788e1 * t3247 * t3255 + 0.26341796731742046394e1 * t1246 * t8504 + 0.26341796731742046394e1 * t411 * t2428 * t10361 * t951 - 0.13170898365871023197e1 * t8481 * t1256 - 0.15805078039045227836e2 * t23398 * t12845 * t3278 + 0.13170898365871023197e1 * t3904 * t2430 - 0.79025390195226139182e1 * t938 * t10297 - 0.13170898365871023197e1 * t938 * t10362 + 0.13170898365871023197e1 * t411 * t10305 * t2453 + 0.65854491829355115987e0 * t28400 * t158 * t415 + 0.26341796731742046394e1 * t411 * t3254 * t8559 + 0.52683593463484092788e1 * t938 * t10300 - 0.13170898365871023197e1 * t1246 * t8560;
    (t28569,)
}
