//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 628/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk628<F: Float>(t322: F, t1020: F, t1127: F, t333: F, t3745: F, t335: F, t337: F, t339: F, t341: F, t1129: F, t1131: F, t1133: F, t1135: F, t1137: F, t343: F, t3729: F) -> (F, F, F, F, F, F, F) {
    let t332 = 0.25e1 < t322;
    let t3747 = t1020 * t1127;
    let t3749 = t333 * t3745;
    let t3753 = t335 * t3745;
    let t3757 = t337 * t3745;
    let t3761 = t339 * t3745;
    let t3765 = t341 * t3745;
    let t3771 = -0.64e0 * t3745 - 0.8704e0 * t3747 - 0.8704e0 * t3749 - 0.9214113627294e1 * t1129 * t1020 - 0.4607056813647e1 * t3753 + 0.367387230261e2 * t1131 * t1020 + 0.122462410087e2 * t3757 - 0.3831420472412e2 * t1133 * t1020 - 0.957855118103e1 * t3761 + 0.1550653405116e2 * t1135 * t1020 + 0.3101306810232e1 * t3765 - 0.2177652951264e1 * t1137 * t1020 - 0.362942158544e0 * t343 * t3745;
    let t3774 = piecewise3(t332, t3729, 0.0);
    (t3749, t3753, t3757, t3761, t3765, t3771, t3774)
}
