//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1242/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1242<F: Float>(t32962: F, t28231: F, t24885: F, t787: F, t1457: F, t2634: F, t28242: F, t28245: F, t11109: F, t22315: F, t2617: F, t7810: F, t8802: F) -> (F, F, F, F, F, F, F, F) {
    let t32963 = F::cast_from(0.29792074959875355558e-1_f64) * t32962;
    let t32968 = F::cast_from(0.31952438294933958064e0_f64) * t28231;
    let t32969 = t787 * t24885;
    let t32970 = t1457 * t2634;
    let t32972 = F::cast_from(0.50050685932590597338e1_f64) * t32969 * t32970;
    let t32973 = F::cast_from(0.25561950635947166452e0_f64) * t28242;
    let t32974 = F::cast_from(0.25561950635947166452e0_f64) * t28245;
    let t32978 = t22315 * t11109;
    let t32979 = F::cast_from(0.38342925953920749676e0_f64) * t32978;
    let t32983 = t7810 * t8802 * t2617;
    (t32963, t32968, t32970, t32972, t32973, t32974, t32979, t32983)
}
