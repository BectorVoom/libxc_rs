//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1198/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1198<F: Float>(t21490: F, t32145: F, t10694: F, t29439: F, t24741: F, t5539: F, t9647: F, t10701: F, t25260: F, t2558: F, t10691: F, t1850: F) -> (F, F, F, F, F, F) {
    let t32147 = F::cast_from(0.51270174867614828558e-2_f64) * t21490 * t32145;
    let t32148 = t29439 * t10694;
    let t32149 = F::cast_from(0.1281754371690370714e-2_f64) * t32148;
    let t32151 = t9647 * t5539 * t24741;
    let t32152 = F::cast_from(0.1281754371690370714e-2_f64) * t32151;
    let t32153 = t29439 * t10701;
    let t32154 = F::cast_from(0.64087718584518535698e-3_f64) * t32153;
    let t32158 = t9647 * t25260 * t2558;
    let t32159 = F::cast_from(0.32043859292259267849e-3_f64) * t32158;
    let t32160 = t1850 * t10691;
    (t32147, t32149, t32152, t32154, t32159, t32160)
}
