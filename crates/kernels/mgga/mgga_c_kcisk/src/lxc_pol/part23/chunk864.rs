//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 864/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk864<F: Float>(t10: F, t3529: F, t1265: F, t4125: F, t373: F, t4128: F, t1275: F, t4120: F, t1233: F, t4023: F, t1229: F, t4080: F, t357: F, t4079: F, t346: F, t1248: F, t3579: F, t3979: F) -> (F, F, F, F, F, F, F, F) {
    let t13538 = t10 * t3529;
    let t13561 = 1.0 / t4125 / t1265;
    let t13565 = 1.0 / t4128 / t373;
    let t13570 = t1275 * t4120;
    let t13574 = t4023 * t1233;
    let t13583 = t1229 * t4080;
    let t13587 = 1.0 / t4079 / t357;
    let t13588 = t346 * t13587;
    let t13595 = t1248 * t3979 * t3579;
    (t13538, t13561, t13565, t13570, t13574, t13583, t13588, t13595)
}
