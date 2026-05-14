//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1251/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1251<F: Float>(t297: F, t33980: F, t294: F, t1152: F, t9896: F, t2068: F, t9406: F, t1156: F, t9895: F, t2071: F, t9575: F, t296: F, t6642: F, t2709: F, t1319: F, t13440: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33981 = t297 * t33980;
    let t33982 = t294 * t33981;
    let t33984 = t1152 * t9896;
    let t33986 = t2068 * t9406;
    let t33987 = t1156 * t9895;
    let t33988 = t294 * t33987;
    let t33990 = t2071 * t9575;
    let t33991 = t294 * t33990;
    let t33993 = t296 * t6642;
    let t33994 = t2709 * t33993;
    let t35049 = 2.0 * t33986;
    let t35843 = t1319 * t13440;
    (t33981, t33982, t33984, t33987, t33988, t33990, t33991, t33993, t33994, t35049, t35843)
}
