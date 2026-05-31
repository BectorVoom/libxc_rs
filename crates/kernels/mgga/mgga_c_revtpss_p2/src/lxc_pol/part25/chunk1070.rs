//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1070/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1070<F: Float>(t12810: F, t12840: F, t3626: F, t1222: F, t1261: F, t1266: F, t12774: F, t12777: F, t12781: F, t12784: F, t12789: F, t12794: F, t12797: F, t12800: F, t12805: F, t12809: F, t12812: F, t12816: F, t12822: F, t12828: F, t12832: F, t12836: F, t3620: F, t3625: F, t3631: F, t3640: F, t3644: F, t3647: F, t3718: F, t3723: F, t5340: F) -> F {
    let t12841 = t12810 * t12840;
    let t12842 = t3626 * t12841;
    let t12845 = -F::cast_from(0.57165357490759649295e-3_f64) * t12774 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t12777 - F::cast_from(0.85748036236139473944e-3_f64) * t3625 * t12781 - F::cast_from(0.85748036236139473944e-3_f64) * t12784 * t3631 + F::cast_from(0.7145669686344956162e-3_f64) * t3625 * t12789 + F::cast_from(0.7145669686344956162e-3_f64) * t3647 * t3620 - t1222 * t12794 / F::cast_from(48.0_f64) + t1222 * t12797 / F::cast_from(72.0_f64) - F::cast_from(0.42874018118069736972e-3_f64) * t12800 * t1266 - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t12805 + F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t12812 + F::cast_from(0.14291339372689912324e-2_f64) * t1261 * t12816 - F::cast_from(0.42874018118069736972e-3_f64) * t3647 * t3640 - F::cast_from(0.14291339372689912324e-3_f64) * t1261 * t12822 - F::cast_from(0.85748036236139473944e-3_f64) * t3647 * t3644 - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t12828 - F::cast_from(0.12862205435420921092e-2_f64) * t12832 * t3723 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t12836 - F::cast_from(0.85748036236139473944e-3_f64) * t5340 * t12842;
    t12845
}
