//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1278/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1278<F: Float>(t20160: F, t32071: F, t9446: F, t32126: F, t3973: F, t32095: F, t3969: F, t1308: F, t13448: F, t388: F, t1292: F, t3491: F, t32022: F, t32042: F, t21499: F, t32018: F) -> (F, F, F, F, F, F, F, F) {
    let t110324 = t20160 * t32071;
    let t110325 = t9446 * t110324;
    let t110335 = t9446 * t3973 * t32126;
    let t110341 = t32095 * t3969;
    let t110347 = t13448 * t388 * t1308;
    let t110351 = t3491 * t1292 * t1308;
    let t110365 = t32022 * t32042;
    let t110384 = t32018 * t21499;
    (t110324, t110325, t110335, t110341, t110347, t110351, t110365, t110384)
}
