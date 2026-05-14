//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1097/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1097<F: Float>(t23220: F, t5486: F, t5006: F, t2634: F, t7624: F, t4419: F, t9177: F, t782: F, t18693: F, t18699: F, t18701: F, t18705: F, t18711: F, t18713: F, t2005: F, t2013: F, t2630: F, t2644: F, t7575: F, t7640: F, t9178: F) -> (F,) {
    let t25124 = t5486 * t23220;
    let t25125 = t5006 * t25124;
    let t25128 = t2634 * t7624;
    let t25130 = t4419 * t9177;
    let t25131 = t782 * t25130;
    let t25133 = -0.5397236614853195164e-1 * t7575 * t2644 - 0.5397236614853195164e-1 * t2630 * t7640 + 0.5397236614853195164e-1 * t2005 * t9178 + t18693 - t18699 + 0.11993859144118211475e-1 * t18701 - t18705 - t18711 + 0.11993859144118211476e-1 * t2013 * t25125 + 0.47975436576472845903e-1 * t25128 + 0.17990788716177317213e-1 * t25131 - t18713;
    (t25133,)
}
