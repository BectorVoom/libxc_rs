//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 961/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk961<F: Float>(t28911: F, t7303: F, t32187: F, t32190: F, t32202: F, t1955: F, t7506: F, t32715: F, t786: F, t7286: F, t2030: F, t32209: F, t32214: F, t32700: F, t32709: F, t32712: F, t32718: F, t32719: F, t7308: F, t8702: F, t8709: F) -> (F, F, F, F, F, F, F, F) {
    let t32720 = t28911 * t7303;
    let t32723 = 0.37645955677973955999e-4 * t32187;
    let t32724 = 0.66934509195437693771e-4 * t32190;
    let t32725 = 0.263521689745817692e-2 * t32202;
    let t32726 = t1955 * t7506;
    let t32729 = t786 * t32715;
    let t32731 = 0.14456046980341999104e-1 * t32729 * t7286;
    let t32732 = 0.57119737665102352616e0 * t32700 * t8709 - 0.225875734067843736e-2 * t32209 - 0.56468933516960933999e-3 * t32214 - t32709 + t32712 - 0.8673628188205199462e0 * t8702 * t7308 - t32718 - 0.11423947533020470523e1 * t32719 * t32720 - t32723 + t32724 - t32725 - 0.8673628188205199462e0 * t32726 * t2030 + t32731;
    (t32720, t32723, t32724, t32725, t32726, t32729, t32731, t32732)
}
