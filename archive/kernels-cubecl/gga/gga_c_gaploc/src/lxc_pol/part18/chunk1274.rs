//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1274/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1274<F: Float>(t32962: F, t28231: F, t24885: F, t787: F, t1457: F, t2634: F, t28242: F, t28245: F, t11109: F, t22315: F, t11001: F, t1445: F, t2061: F, t2070: F, t2201: F, t28249: F, t28259: F, t32387: F, t32951: F, t32953: F, t32955: F, t32958: F, t32960: F) -> (F, F) {
    let t32963 = F::cast_from(0.29792074959875355558e-1_f64) * t32962;
    let t32968 = F::cast_from(0.31952438294933958064e0_f64) * t28231;
    let t32969 = t787 * t24885;
    let t32970 = t1457 * t2634;
    let t32972 = F::cast_from(0.50050685932590597338e1_f64) * t32969 * t32970;
    let t32973 = F::cast_from(0.25561950635947166452e0_f64) * t28242;
    let t32974 = F::cast_from(0.25561950635947166452e0_f64) * t28245;
    let t32978 = t22315 * t11109;
    let t32979 = F::cast_from(0.38342925953920749676e0_f64) * t32978;
    let t32980 = -t32951 + t32953 + t32955 - t32958 - t32960 + t32963 + F::cast_from(0.71500979903700853338e0_f64) * t2070 * t11001 + F::cast_from(0.35750489951850426669e0_f64) * t2061 * t11001 - t32968 - t32972 - t32973 + t32974 - t28249 - t28259 - F::cast_from(0.46011511144704899612e1_f64) * t2201 * t1445 * t32387 - t32979;
    (t32970, t32980)
}
