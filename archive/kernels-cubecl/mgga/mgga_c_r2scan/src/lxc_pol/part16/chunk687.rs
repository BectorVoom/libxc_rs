//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 687/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk687<F: Float>(t234: F, t5305: F, t148: F, t1683: F, t5245: F, t22: F, t502: F, t1712: F, t5: F, t518: F, t586: F, t4849: F, t4850: F, t4851: F, t4852: F, t4853: F) -> (F, F, F, F, F, F, F) {
    let t5307 = F::cast_from(0.51947577317044391277e2_f64) * t234 * t5305;
    let t5308 = t148 * t1683;
    let t5309 = t5308 * t5245;
    let t5311 = t22 * t502;
    let t5312 = t1712 * t5311;
    let t5314 = t5 * t518;
    let t5315 = t586 * t5314;
    let t5317 = -F::cast_from(0.17261666666666666667e1_f64) * t5309 + F::cast_from(0.11507777777777777778e1_f64) * t5312 - F::cast_from(0.53702962962962962964e1_f64) * t5315 - t4849 + t4850 - t4851 - t4852 - t4853;
    (t5307, t5309, t5311, t5312, t5314, t5315, t5317)
}
