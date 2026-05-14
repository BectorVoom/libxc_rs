//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1048/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1048<F: Float>(t19950: F, t492: F, t1506: F, t19801: F, t6369: F, t6368: F, t14304: F, t6357: F, t14340: F, t2275: F, t6313: F, t19005: F, t4231: F, t4230: F, t4215: F, t6377: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21011 = t19950 * t492;
    let t21012 = t21011 * t1506;
    let t21014 = t6369 * t19801;
    let t21015 = t6368 * t21014;
    let t21017 = t14304 * t6357;
    let t21019 = t14340 * t2275;
    let t21022 = t14304 * t6313;
    let t21024 = t4231 * t19005;
    let t21025 = t4230 * t21024;
    let t21027 = t4215 * t6377;
    (t21012, t21014, t21015, t21017, t21019, t21022, t21024, t21025, t21027)
}
