//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 895/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk895<F: Float>(t12277: F, t1264: F, t247: F, t12273: F, t1284: F, t3555: F, t3624: F, t12803: F, t3629: F, t3626: F, t1121: F, t3603: F, t606: F, t12810: F, t1222: F, t1261: F, t1266: F, t12774: F, t12777: F, t12781: F, t12784: F, t12789: F, t12794: F, t12797: F, t12800: F, t12805: F, t12809: F, t12812: F, t12816: F, t3620: F, t3625: F, t3631: F, t3640: F, t3644: F, t3647: F, t3718: F, t3723: F, t5340: F) -> (F,) {
    let t12822 = t247 * t1264 * t12277;
    let t12828 = t247 * t1264 * t12273;
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    let t12835 = t12803 * t3629;
    let t12836 = t3626 * t12835;
    let t12839 = t3603 * t1121;
    let t12840 = t12839 * t606;
    let t12841 = t12810 * t12840;
    let t12842 = t3626 * t12841;
    let t12845 = -0.57165357490759649295e-3 * t12774 - 0.42874018118069736972e-3 * t3625 * t12777 - 0.85748036236139473944e-3 * t3625 * t12781 - 0.85748036236139473944e-3 * t12784 * t3631 + 0.7145669686344956162e-3 * t3625 * t12789 + 0.7145669686344956162e-3 * t3647 * t3620 - t1222 * t12794 / 48.0 + t1222 * t12797 / 72.0 - 0.42874018118069736972e-3 * t12800 * t1266 - 0.64311027177104605458e-3 * t3718 * t12805 + 0.64311027177104605458e-3 * t12809 * t12812 + 0.14291339372689912324e-2 * t1261 * t12816 - 0.42874018118069736972e-3 * t3647 * t3640 - 0.14291339372689912324e-3 * t1261 * t12822 - 0.85748036236139473944e-3 * t3647 * t3644 - 0.85748036236139473944e-3 * t1261 * t12828 - 0.12862205435420921092e-2 * t12832 * t3723 - 0.42874018118069736972e-3 * t3625 * t12836 - 0.85748036236139473944e-3 * t5340 * t12842;
    (t12845,)
}
