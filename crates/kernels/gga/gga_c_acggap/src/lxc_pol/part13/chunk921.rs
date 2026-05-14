//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 921/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk921<F: Float>(t1165: F, t4762: F, t7564: F, t8600: F, t30308: F, t30310: F, t30314: F, t30319: F, t2304: F, t7610: F, t1988: F, t8561: F, t30316: F, t34171: F, t34173: F, t34176: F, t34179: F, t34183: F, t34189: F, t34193: F, t34197: F, t34201: F, t34204: F) -> (F,) {
    let t34208 = t7564 * t1165 * t8600 * t4762;
    let t34210 = 77.0 / 288.0 * t30308;
    let t34211 = 77.0 / 864.0 * t30310;
    let t34212 = 0.7640625e-2 * t30314;
    let t34214 = 0.16006300097412701803e-1 * t30319;
    let t34215 = t7610 * t2304;
    let t34217 = t1988 * t8561;
    let t34218 = 0.62896184579208304136e-3 * t34217;
    let t34219 = -t34171 + t34173 + t34176 - 0.10482697429868050689e-2 * t34179 - 0.47172138434406228102e-2 * t34183 - 0.62896184579208304136e-2 * t34189 + 0.18868855373762491241e-2 * t34193 + 0.15724046144802076034e-2 * t34197 - 0.23586069217203114051e-2 * t34201 - 0.80031500487063509015e-2 * t34204 - 0.37737710747524982482e-2 * t34208 - t34210 - t34211 - t34212 - 0.31448092289604152068e-3 * t30316 + t34214 - 0.31448092289604152068e-3 * t34215 - t34218;
    (t34219,)
}
