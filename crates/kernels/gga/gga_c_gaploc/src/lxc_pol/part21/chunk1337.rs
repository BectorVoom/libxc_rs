//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1337/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1337<F: Float>(t34830: F, t18736: F, t2365: F, t25575: F, t25735: F, t7025: F, t10402: F, t20675: F, t204: F, t2476: F, t34371: F, t10310: F, t1429: F, t549: F) -> (F, F, F, F, F, F) {
    let t34831 = F::cast_from(0.59584149919750711116e-1_f64) * t34830;
    let t34838 = t18736 * t2365 * t25575;
    let t34839 = F::cast_from(0.29792074959875355558e-1_f64) * t34838;
    let t34841 = t7025 * t2365 * t25735;
    let t34842 = F::cast_from(0.29792074959875355558e-1_f64) * t34841;
    let t34854 = t20675 * t10402;
    let t34855 = F::cast_from(0.38342925953920749676e0_f64) * t34854;
    let t34860 = F::cast_from(0.92023022289409799224e1_f64) * t2476 * t204 * t34371;
    let t34862 = t1429 * t549 * t10310;
    (t34831, t34839, t34842, t34855, t34860, t34862)
}
