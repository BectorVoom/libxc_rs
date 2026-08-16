//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1225/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1225(t12803: f64, t3629: f64, t3626: f64, t1121: f64, t3603: f64, t606: f64, t12810: f64, t1222: f64, t1261: f64, t1266: f64, t12774: f64, t12777: f64, t12781: f64, t12784: f64, t12789: f64, t12794: f64, t12797: f64, t12800: f64, t12805: f64, t12809: f64, t12812: f64, t12816: f64, t12822: f64, t12828: f64, t12832: f64, t3620: f64, t3625: f64, t3631: f64, t3640: f64, t3644: f64, t3647: f64, t3718: f64, t3723: f64, t5340: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12835 = t12803 * t3629;
    let t12836 = t3626 * t12835;
    let t12839 = t3603 * t1121;
    let t12840 = t12839 * t606;
    let t12841 = t12810 * t12840;
    let t12842 = t3626 * t12841;
    let t12845 = -0.57165357490759649295e-3_f64 * t12774 - 0.42874018118069736972e-3_f64 * t3625 * t12777 - 0.85748036236139473944e-3_f64 * t3625 * t12781 - 0.85748036236139473944e-3_f64 * t12784 * t3631 + 0.7145669686344956162e-3_f64 * t3625 * t12789 + 0.7145669686344956162e-3_f64 * t3647 * t3620 - t1222 * t12794 / 48.0_f64 + t1222 * t12797 / 72.0_f64 - 0.42874018118069736972e-3_f64 * t12800 * t1266 - 0.64311027177104605458e-3_f64 * t3718 * t12805 + 0.64311027177104605458e-3_f64 * t12809 * t12812 + 0.14291339372689912324e-2_f64 * t1261 * t12816 - 0.42874018118069736972e-3_f64 * t3647 * t3640 - 0.14291339372689912324e-3_f64 * t1261 * t12822 - 0.85748036236139473944e-3_f64 * t3647 * t3644 - 0.85748036236139473944e-3_f64 * t1261 * t12828 - 0.12862205435420921092e-2_f64 * t12832 * t3723 - 0.42874018118069736972e-3_f64 * t3625 * t12836 - 0.85748036236139473944e-3_f64 * t5340 * t12842;
    (t12835, t12836, t12839, t12840, t12841, t12842, t12845)
}
