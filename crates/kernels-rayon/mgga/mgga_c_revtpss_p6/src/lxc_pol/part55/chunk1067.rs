//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1067/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1067(t28911: f64, t7303: f64, t32187: f64, t32190: f64, t32202: f64, t1955: f64, t7506: f64, t32715: f64, t786: f64, t7286: f64, t2030: f64, t32209: f64, t32214: f64, t32700: f64, t32709: f64, t32712: f64, t32718: f64, t32719: f64, t7308: f64, t8702: f64, t8709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32720 = t28911 * t7303;
    let t32723 = 0.37645955677973955999e-4_f64 * t32187;
    let t32724 = 0.66934509195437693771e-4_f64 * t32190;
    let t32725 = 0.263521689745817692e-2_f64 * t32202;
    let t32726 = t1955 * t7506;
    let t32729 = t786 * t32715;
    let t32731 = 0.14456046980341999104e-1_f64 * t32729 * t7286;
    let t32732 = 0.57119737665102352616e0_f64 * t32700 * t8709 - 0.225875734067843736e-2_f64 * t32209 - 0.56468933516960933999e-3_f64 * t32214 - t32709 + t32712 - 0.8673628188205199462e0_f64 * t8702 * t7308 - t32718 - 0.11423947533020470523e1_f64 * t32719 * t32720 - t32723 + t32724 - t32725 - 0.8673628188205199462e0_f64 * t32726 * t2030 + t32731;
    (t32720, t32723, t32724, t32725, t32726, t32729, t32731, t32732)
}
