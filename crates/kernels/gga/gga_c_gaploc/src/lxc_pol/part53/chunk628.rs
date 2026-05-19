//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 628/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk628<F: Float>(t10942: F, t9800: F, t3503: F, t4614: F, t2087: F, t3447: F, t833: F, t3483: F, t813: F, t2194: F, t3484: F, t8528: F, t935: F) -> (F, F, F, F, F, F) {
    let t10943 = t9800 * t10942;
    let t10944 = F::cast_from(0.9585731488480187419e0_f64) * t10943;
    let t10951 = t4614 * t3503;
    let t10953 = F::cast_from(0.92023022289409799224e1_f64) * t2087 * t10951;
    let t10961 = t4614 * t3447;
    let t10963 = F::cast_from(0.15337170381568299871e2_f64) * t833 * t10961;
    let t10964 = t4614 * t3483;
    let t10966 = F::cast_from(0.61348681526273199483e1_f64) * t813 * t10964;
    let t10971 = F::cast_from(0.46011511144704899612e1_f64) * t2194 * t3484;
    let t10972 = t8528 * t935;
    (t10944, t10953, t10963, t10966, t10971, t10972)
}
