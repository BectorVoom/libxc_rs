//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1035/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1035<F: Float>(t7219: F, t7253: F, t25: F, t8815: F, t1773: F, t8821: F, t17248: F, t17290: F, t17726: F, t1787: F, t23326: F, t23338: F, t2466: F, t4989: F, t7208: F, t7235: F, t7258: F, t7270: F, t8816: F, t8822: F) -> (F,) {
    let t23840 = t7219 * t7253;
    let t23842 = t25 * t8815;
    let t23843 = t1773 * t23842;
    let t23857 = t25 * t8821;
    let t23858 = t1773 * t23857;
    let t23865 = 0.95950873152945691807e-1 * t23840 + 0.35981577432354634427e-1 * t23843 - 0.12793449753726092241e0 * t17248 * t7235 + 0.95950873152945691807e-1 * t17248 * t7258 - 0.10794473229706390328e0 * t7208 * t7270 - 0.5397236614853195164e-1 * t4989 * t8822 + 0.28785261945883707542e0 * t23338 * t1787 - 0.10794473229706390328e0 * t17290 * t2466 - 0.17990788716177317213e-1 * t23858 - 0.5397236614853195164e-1 * t23326 * t1787 + 0.10794473229706390328e0 * t4989 * t8816 + 0.95950873152945691804e-1 * t17726;
    (t23865,)
}
