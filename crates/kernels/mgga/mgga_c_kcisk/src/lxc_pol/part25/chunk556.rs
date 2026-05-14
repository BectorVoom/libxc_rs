//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 556/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk556<F: Float>(t1783: F, t5038: F, t1310: F, t1773: F, t1778: F, t1787: F, t4984: F, t4987: F, t4989: F, t4997: F, t5000: F, t5003: F, t5009: F, t5013: F, t5017: F, t5022: F, t5026: F, t5034: F, t664: F) -> (F, F, F) {
    let t5039 = t1783 * t5038;
    let t5040 = t1310 * t5039;
    let t5043 = 0.5397236614853195164e-1 * t4984 * t664 + 0.35981577432354634426e-1 * t4987 + 0.35981577432354634426e-1 * t4989 * t1778 - 0.10794473229706390328e0 * t4989 * t1787 - t4997 + 0.11993859144118211475e-1 * t5000 - 0.35981577432354634426e-1 * t5003 + 0.23987718288236422951e-1 * t1773 * t5009 - 0.35981577432354634426e-1 * t5013 * t5017 - 0.35981577432354634426e-1 * t1773 * t5022 + 0.17990788716177317213e-1 * t1773 * t5026 + 0.10794473229706390328e0 * t1773 * t5034 - 0.5397236614853195164e-1 * t1773 * t5040;
    (t5039, t5040, t5043)
}
