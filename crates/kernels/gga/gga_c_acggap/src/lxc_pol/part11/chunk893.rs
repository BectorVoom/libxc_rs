//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 893/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk893<F: Float>(t33743: F, t33744: F, t620: F, t524: F, t943: F, t944: F, t1222: F, t1264: F, t1620: F, t2146: F, t2147: F, t2159: F, t2331: F, t32001: F, t32006: F, t32012: F, t33715: F, t33726: F, t33727: F, t33735: F, t33739: F, t7931: F, t7932: F, t8001: F, t8400: F, t8403: F, t9010: F, t9033: F, t9058: F) -> (F, F, F) {
    let t33747 = 0.10408353825846239354e2 * t33743 * t620 * t33744;
    let t33750 = t524 * t943;
    let t33751 = t33750 * t944;
    let t33755 = 0.13170898365871023197e1 * t33715 + 0.13170898365871023197e1 * t9010 * t1222 + 0.13170898365871023197e1 * t32001 + 0.26341796731742046394e1 * t8001 * t1620 + 0.34694512752820797848e1 * t32006 - t33726 + 0.8673628188205199462e0 * t33727 * t8403 + 0.8673628188205199462e0 * t32012 + 0.8673628188205199462e0 * t2146 * t2147 * t2331 * t1264 - 0.8673628188205199462e0 * t8400 * t9033 * t33735 - 0.8673628188205199462e0 * t7931 * t7932 * t33739 - t33747 - 0.8673628188205199462e0 * t9058 * t2159 - 0.26020884564615598386e1 * t8400 * t9033 * t33751;
    (t33750, t33751, t33755)
}
