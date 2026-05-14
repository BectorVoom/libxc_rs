//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1105/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1105<F: Float>(t1457: F, t2634: F, t32969: F, t28242: F, t28245: F, t11109: F, t22315: F, t2617: F, t7810: F, t8802: F, t3005: F, t7344: F, t32435: F, t5241: F, t5640: F, t590: F) -> (F, F, F, F, F, F, F, F) {
    let t32970 = t1457 * t2634;
    let t32972 = 0.50050685932590597338e1 * t32969 * t32970;
    let t32973 = 0.25561950635947166452e0 * t28242;
    let t32974 = 0.25561950635947166452e0 * t28245;
    let t32978 = t22315 * t11109;
    let t32979 = 0.38342925953920749676e0 * t32978;
    let t32983 = t7810 * t8802 * t2617;
    let t32984 = 0.38342925953920749676e0 * t32983;
    let t32986 = t7810 * t3005 * t7344;
    let t32987 = 0.19171462976960374838e0 * t32986;
    let t32991 = 0.30674340763136599742e1 * t5640 * t5241 * t32435 * t590;
    (t32970, t32972, t32973, t32974, t32979, t32984, t32987, t32991)
}
