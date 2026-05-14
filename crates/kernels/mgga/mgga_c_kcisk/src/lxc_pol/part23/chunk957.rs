//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 957/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk957<F: Float>(t19413: F, t19419: F, t1433: F, t3532: F, t1364: F, t5626: F, t1354: F, t2083: F, t3593: F, t3564: F, t425: F, t5703: F, t1175: F, t3587: F, t5948: F, t1390: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19420 = t19419 * t19413;
    let t19423 = t1433 * t3532;
    let t19424 = t5626 * t1364;
    let t19425 = t19423 * t19424;
    let t19434 = t1354 * t2083;
    let t19435 = t19434 * t3593;
    let t19436 = t3564 * t19435;
    let t19439 = t425 * t5703;
    let t19440 = t19439 * t1175;
    let t19441 = t3564 * t19440;
    let t19444 = t5948 * t3587;
    let t19445 = t3564 * t19444;
    let t19450 = t1433 * t1390;
    (t19420, t19424, t19425, t19435, t19436, t19440, t19441, t19444, t19445, t19450)
}
