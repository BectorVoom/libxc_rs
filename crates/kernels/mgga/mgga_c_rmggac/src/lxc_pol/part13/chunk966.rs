//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 966/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk966<F: Float>(t874: F, t9486: F, t352: F, t2447: F, t4616: F, t876: F, t42023: F, t42026: F, t4905: F, t9540: F, t42044: F, t42057: F, t1356: F, t29838: F, t36893: F, t38107: F, t42011: F, t42032: F, t42034: F, t42042: F, t42050: F, t42055: F, t42059: F, t4985: F, t8281: F) -> (F, F, F, F) {
    let t43970 = t874 * t9486;
    let t43971 = t43970 * t352;
    let t43974 = t4616 * t2447;
    let t43975 = t43974 * t876;
    let t43978 = 0.162600798888400151e-2 * t42023;
    let t43979 = 0.162600798888400151e-2 * t42026;
    let t43981 = t9540 * t4905;
    let t43987 = 0.11918087970123395032e-3 * t42044;
    let t43990 = 0.87811105813667929469e0 * t42057;
    let t43993 = 0.23942587439980034662e-4 * t42011 + 0.11974241701863808564e0 * t4985 * t8281 + 0.79828278012425390428e-1 * t1356 * t43971 - 0.11974241701863808564e0 * t1356 * t43975 - t43978 - t43979 - 0.39726959900411316772e-4 * t36893 + 0.95793933614910468512e0 * t29838 * t43981 + 0.30487649791575028312e-3 * t42032 - 0.47896966807455234256e0 * t42034 + 0.60975299583150056624e-3 * t42042 + t43987 - 0.5107751987195740728e-4 * t42050 + 0.5107751987195740728e-4 * t42055 + t43990 + 0.5987120850931904282e-1 * t42059 - 0.4726e1 * t38107;
    (t43971, t43975, t43981, t43993)
}
