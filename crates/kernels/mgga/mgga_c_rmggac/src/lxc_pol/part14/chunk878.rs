//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 878/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk878<F: Float>(t4895: F, t665: F, t39879: F, t5271: F, t262: F, t40802: F, t7835: F, t35815: F, t39662: F, t39666: F, t7788: F, t40833: F, t36254: F, t40805: F, t7782: F, t40808: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t40960 = t665 * t4895;
    let t40963 = t5271 * t39879;
    let t40965 = t262 * t40802;
    let t40966 = t7835 * t40965;
    let t40967 = 0.36366215538993788972e-1 * t40966;
    let t40968 = t35815 * t39662;
    let t40970 = t7788 * t39666;
    let t40972 = t262 * t40833;
    let t40973 = t36254 * t40972;
    let t40975 = t262 * t40805;
    let t40976 = t7782 * t40975;
    let t40978 = t262 * t40808;
    (t40960, t40963, t40965, t40967, t40968, t40970, t40972, t40973, t40975, t40976, t40978)
}
