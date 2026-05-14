//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1080/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1080<F: Float>(t2131: F, t2147: F, t309: F, t9413: F, t1659: F, t8331: F, t33698: F, t33699: F, t638: F, t315: F, t323: F, t9367: F, t119: F, t1264: F, t150: F, t1620: F, t187: F, t2146: F, t2222: F, t2394: F, t31965: F, t32124: F, t33180: F, t33185: F, t33198: F, t36547: F, t38001: F, t5332: F, t7912: F, t8004: F, t8306: F, t8316: F, t9145: F, t9165: F) -> (F,) {
    let t38241 = 0.34694512752820797848e1 * t2131 * t2147 * t9413 * t309;
    let t38251 = 0.13170898365871023197e1 * t8331 * t1659;
    let t38256 = 0.10408353825846239354e2 * t33698 * t638 * t33699;
    let t38259 = 0.13170898365871023197e1 * t315 * t9367 * t323;
    let t38270 = 0.17347256376410398924e1 * t33180 + t33185 + t38241 - 0.26020884564615598386e1 * t2146 * t8004 * t2394 * t1264 - 0.65854491829355115987e0 * t2222 * t5332 + 0.8673628188205199462e0 * t7912 * t9145 - t38251 - 0.17347256376410398924e1 * t31965 * t9165 + t38256 - t38259 + 0.65854491829355115987e0 * t119 * t38001 * t150 * t187 + 0.26020884564615598386e1 * t32124 * t8306 * t36547 + 0.26341796731742046394e1 * t8316 * t1620 - 0.17347256376410398924e1 * t33198;
    (t38270,)
}
