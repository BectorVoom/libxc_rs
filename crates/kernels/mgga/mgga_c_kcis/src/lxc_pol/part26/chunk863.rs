//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 863/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk863<F: Float>(t18210: F, t6178: F, t1599: F, t1369: F, t2470: F, t6164: F, t17259: F, t17267: F, t17274: F, t17276: F, t2093: F, t4413: F) -> (F, F, F, F, F, F, F, F) {
    let t18211 = t18210 * t6178;
    let t18213 = t1599 * t18211 / F::cast_from(144.0_f64);
    let t18221 = t2470 * t1369;
    let t18222 = t18221 * t6164;
    let t18223 = t1599 * t18222;
    let t18244 = F::cast_from(0.23214722222222222222e-2_f64) * t17259;
    let t18246 = F::cast_from(0.25794135802469135802e-2_f64) * t17267;
    let t18248 = F::cast_from(0.30952962962962962962e-2_f64) * t17274;
    let t18249 = F::cast_from(0.10317654320987654321e-2_f64) * t17276;
    let t18253 = t2093 * t4413;
    (t18213, t18221, t18223, t18244, t18246, t18248, t18249, t18253)
}
