//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 909/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk909<F: Float>(t4811: F, t6686: F, t6709: F, t3521: F, t7025: F, t7001: F, t11417: F, t708: F, t7005: F, t7009: F, t7013: F, t1814: F, t4629: F, t1648: F, t2487: F, t7047: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16687 = t4811 * t6686;
    let t16688 = 0.66327777777777777776e-2 * t16687;
    let t16702 = t4811 * t6709;
    let t16729 = 0.13140859333333333333e-2 * t3521 * t7025;
    let t16759 = 0.14600954814814814815e-2 * t3521 * t7001;
    let t16763 = t11417 * t708;
    let t16779 = 0.13140859333333333334e-2 * t3521 * t7005;
    let t16781 = 0.8760572888888888889e-3 * t3521 * t7009;
    let t16784 = 0.17521145777777777778e-2 * t3521 * t7013;
    let t16804 = t4629 * t1814;
    let t16805 = t2487 * t1648;
    let t16810 = t3521 * t7047;
    (t16687, t16688, t16702, t16729, t16759, t16763, t16779, t16781, t16784, t16804, t16805, t16810)
}
