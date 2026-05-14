//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 855/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk855<F: Float>(t23286: F, t23320: F, t23872: F, t28320: F, t28327: F, t28334: F, t28703: F, t28706: F, t28711: F, t28715: F, t28719: F, t28722: F, t24561: F, t2647: F, t1994: F, t23874: F, t23876: F, t23878: F, t23880: F, t23894: F, t28726: F, t28732: F, t28752: F, t28758: F, t28762: F, t28765: F) -> (F, F, F) {
    let t29971 = -0.52233124999999999998e-2 * t28320 - 0.46429444444444444443e-2 * t23286 - 0.34822083333333333333e-2 * t28327 + 0.13928833333333333333e-1 * t28334 + 0.17411041666666666666e-2 * t28703 - 0.13928833333333333333e-1 * t28706 + 0.34822083333333333333e-2 * t23320 - 0.46429444444444444443e-2 * t23872 - 0.11607361111111111111e-2 * t28711 - 0.51072388888888888887e-1 * t28715 + 0.34048259259259259259e-1 * t28719 - 0.18571777777777777778e-1 * t28722;
    let t29981 = t24561 * t2647;
    let t29988 = -0.92858888888888888888e-2 * t28726 + 0.10446625e-1 * t28732 + 0.23214722222222222222e-2 * t23874 - 0.69644166666666666665e-2 * t23876 - 0.77382407407407407405e-3 * t23878 - 0.12381185185185185185e-1 * t23880 - 0.34822083333333333333e-2 * t23894 + 0.579e0 * t1994 * t29981 + 0.10446625e-1 * t28752 + 0.11607361111111111111e-2 * t28758 + 0.51588271604938271605e-2 * t28762 + 0.34822083333333333333e-2 * t28765;
    (t29971, t29981, t29988)
}
